//! MCP server functional tests - exercises tool handlers via JSON-RPC stdio

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// Send JSON-RPC requests to the MCP server and collect responses
fn run_mcp_session(requests: &[&str]) -> Vec<String> {
    let bin_path = env!("CARGO_BIN_EXE_draftkit");

    let mut child = Command::new(bin_path)
        .args(["serve"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn draftkit serve");

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");

    // Count requests that expect responses (those with "id" field)
    let expected_responses = requests.iter().filter(|r| r.contains("\"id\"")).count();

    // Start reading responses in a separate thread
    let reader_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .take(expected_responses)
            .collect::<Vec<String>>()
    });

    // Send all requests with small delays to allow processing
    for request in requests {
        writeln!(stdin, "{request}").expect("Failed to write request");
        stdin.flush().expect("Failed to flush stdin");
        thread::sleep(Duration::from_millis(50));
    }

    // Wait for reader thread to collect responses
    let responses = reader_thread.join().expect("Reader thread panicked");

    // Close stdin to signal EOF (this allows the server to exit gracefully)
    drop(stdin);

    // Wait briefly for the server to process the EOF and exit
    thread::sleep(Duration::from_millis(100));

    // Wait for process to exit (this allows coverage data to be written)
    // Use a short timeout and only kill if necessary
    match child.try_wait() {
        Ok(Some(_)) => {} // Already exited
        _ => {
            // Give it a bit more time to exit gracefully
            thread::sleep(Duration::from_millis(200));
            // If still running, kill it (coverage may be lost, but test completes)
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    responses
}

#[test]
fn mcp_initialize_handshake() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    assert!(!responses.is_empty(), "Should receive initialize response");

    let resp = &responses[0];
    assert!(resp.contains("\"result\""), "Should have result field");
    assert!(
        resp.contains("serverInfo") || resp.contains("capabilities"),
        "Should have server info or capabilities"
    );
}

#[test]
fn mcp_tools_list() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ];

    let responses = run_mcp_session(requests);

    // Find tools/list response
    let tools_resp = responses.iter().find(|r| r.contains("get_tailwind_docs"));
    assert!(
        tools_resp.is_some(),
        "Should list get_tailwind_docs tool: {responses:?}"
    );
}

#[test]
fn mcp_get_tailwind_docs() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"flexbox"}}}"#,
    ];

    let responses = run_mcp_session(requests);

    // Should have response with flexbox content
    let has_flexbox = responses
        .iter()
        .any(|r| r.contains("flex") || r.contains("Flexbox"));
    assert!(has_flexbox, "Should return flexbox docs: {responses:?}");
}

#[test]
fn mcp_get_tailwind_docs_v3() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"flexbox","version":"v3"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_content = responses.iter().any(|r| r.contains("flex"));
    assert!(has_content, "Should return v3 flexbox docs: {responses:?}");
}

#[test]
fn mcp_get_tailwind_docs_list_topics() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"list"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_topics = responses
        .iter()
        .any(|r| r.contains("flexbox") && r.contains("grid"));
    assert!(has_topics, "Should list available topics: {responses:?}");
}

#[test]
fn mcp_search_components() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_components","arguments":{"query":"button"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Just verify we get a response (may be empty if no components)
    assert!(!responses.is_empty(), "Should receive search response");
}

#[test]
#[cfg(feature = "embedded-data")]
fn mcp_get_catalyst_component() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_catalyst_component","arguments":{"name":"button"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_button = responses.iter().any(|r| r.contains("Button"));
    assert!(has_button, "Should return button component: {responses:?}");
}

#[test]
#[cfg(feature = "embedded-data")]
fn mcp_get_catalyst_component_javascript() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_catalyst_component","arguments":{"name":"button","language":"javascript"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_button = responses.iter().any(|r| r.contains("Button"));
    assert!(
        has_button,
        "Should return JS button component: {responses:?}"
    );
}

#[test]
#[cfg(feature = "embedded-data")]
fn mcp_list_catalyst_components() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_catalyst_components","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_list = responses
        .iter()
        .any(|r| r.contains("button") && r.contains("dialog"));
    assert!(has_list, "Should list catalyst components: {responses:?}");
}

#[test]
fn mcp_get_elements_docs_dialog() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_elements_docs","arguments":{"component":"dialog"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_dialog = responses
        .iter()
        .any(|r| r.contains("dialog") || r.contains("Dialog"));
    assert!(
        has_dialog,
        "Should return dialog element docs: {responses:?}"
    );
}

#[test]
#[cfg(feature = "embedded-data")]
fn mcp_get_elements_docs_overview() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_elements_docs","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Overview should mention available components
    let has_overview = responses
        .iter()
        .any(|r| r.contains("Elements") && r.contains("Autocomplete"));
    assert!(
        has_overview,
        "Should return elements overview: {responses:?}"
    );
}

#[test]
fn mcp_get_summary() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_summary","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_summary = responses
        .iter()
        .any(|r| r.contains("draftkit") || r.contains("Tailwind"));
    assert!(has_summary, "Should return server summary: {responses:?}");
}

#[test]
fn mcp_prompts_list() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"prompts/list"}"#,
    ];

    let responses = run_mcp_session(requests);
    assert!(!responses.is_empty(), "Should list prompts");
}

#[test]
fn mcp_resources_list() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"resources/list"}"#,
    ];

    let responses = run_mcp_session(requests);
    assert!(!responses.is_empty(), "Should list resources");
}

#[test]
fn mcp_error_nonexistent_topic() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"nonexistent-xyz-12345"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Should get a response (either error or "not found" message)
    assert!(!responses.is_empty(), "Should handle nonexistent topic");
}

#[test]
fn mcp_error_invalid_version() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"flexbox","version":"v5"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses
        .iter()
        .any(|r| r.contains("error") && r.contains("Invalid version"));
    assert!(
        has_error,
        "Should return invalid version error: {responses:?}"
    );
}

#[test]
fn mcp_error_nonexistent_catalyst() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_catalyst_component","arguments":{"name":"nonexistent-xyz-12345"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    assert!(
        !responses.is_empty(),
        "Should handle nonexistent catalyst component"
    );
}

#[test]
fn mcp_get_component() {
    // Use a component ID from embedded data (path-like format)
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_component","arguments":{"id":"application-ui/application-shells/sidebar-layouts/simple-sidebar","framework":"react","mode":"light"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Should return component code with className or similar React patterns
    let has_code = responses
        .iter()
        .any(|r| r.contains("className") || r.contains("Component"));
    assert!(has_code, "Should return component code: {responses:?}");
}

#[test]
fn mcp_search_no_results() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_components","arguments":{"query":"zzzznonexistentzzzz12345"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_no_results = responses.iter().any(|r| r.contains("No components found"));
    assert!(
        has_no_results,
        "Should return no results message: {responses:?}"
    );
}

#[test]
fn mcp_search_with_category() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_components","arguments":{"query":"button","category":"Application UI"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Should return results or no results message
    let has_response = responses.iter().any(|r| r.contains("result"));
    assert!(has_response, "Should return search results: {responses:?}");
}

#[test]
#[cfg(feature = "embedded-data")]
fn mcp_list_categories() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_categories","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_categories = responses
        .iter()
        .any(|r| r.contains("Application UI") || r.contains("Marketing"));
    assert!(
        has_categories,
        "Should list component categories: {responses:?}"
    );
}

#[test]
fn mcp_list_elements() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_elements","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_elements = responses
        .iter()
        .any(|r| r.contains("dialog") || r.contains("Dialog"));
    assert!(
        has_elements,
        "Should list element components: {responses:?}"
    );
}

#[test]
fn mcp_get_template_info() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_template_info","arguments":{}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Template info should return result (may be empty if no templates)
    let has_response = responses.iter().any(|r| r.contains("result"));
    assert!(has_response, "Should return template info: {responses:?}");
}

#[test]
fn mcp_prompts_get() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"prompts/get","params":{"name":"implement-ui","arguments":{"description":"login form","framework":"react"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_prompt = responses
        .iter()
        .any(|r| r.contains("messages") || r.contains("implement"));
    assert!(has_prompt, "Should return prompt content: {responses:?}");
}

#[test]
fn mcp_resources_read() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"resources/read","params":{"uri":"draftkit://summary"}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_summary = responses
        .iter()
        .any(|r| r.contains("draftkit") || r.contains("Tailwind"));
    assert!(has_summary, "Should read summary resource: {responses:?}");
}

#[test]
fn mcp_error_invalid_catalyst_language() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_catalyst_component","arguments":{"name":"button","language":"python"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses
        .iter()
        .any(|r| r.contains("error") && r.contains("Invalid language"));
    assert!(
        has_error,
        "Should return invalid language error: {responses:?}"
    );
}

#[test]
fn mcp_error_element_not_found() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_elements_docs","arguments":{"component":"nonexistent-xyz"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses
        .iter()
        .any(|r| r.contains("error") && r.contains("not found"));
    assert!(
        has_error,
        "Should return element not found error: {responses:?}"
    );
}

#[test]
fn mcp_get_template_by_name() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_template_info","arguments":{"name":"Spotlight"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_template = responses
        .iter()
        .any(|r| r.contains("Spotlight") && r.contains("Personal"));
    assert!(
        has_template,
        "Should return Spotlight template: {responses:?}"
    );
}

#[test]
fn mcp_error_template_not_found() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_template_info","arguments":{"name":"NonexistentTemplate"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses
        .iter()
        .any(|r| r.contains("error") && r.contains("not found"));
    assert!(
        has_error,
        "Should return template not found error: {responses:?}"
    );
}

#[test]
fn mcp_prompts_get_explain_utility() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"prompts/get","params":{"name":"explain-utility","arguments":{"utility":"flex"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_prompt = responses
        .iter()
        .any(|r| r.contains("messages") && r.contains("flex"));
    assert!(
        has_prompt,
        "Should return explain-utility prompt: {responses:?}"
    );
}

#[test]
fn mcp_error_unknown_prompt() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"prompts/get","params":{"name":"nonexistent-prompt"}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses.iter().any(|r| r.contains("error"));
    assert!(
        has_error,
        "Should return unknown prompt error: {responses:?}"
    );
}

#[test]
fn mcp_tailwind_docs_fuzzy_match() {
    // Use a typo that might trigger fuzzy matching suggestions
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_tailwind_docs","arguments":{"topic":"flex"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // "flex" should either find content or suggest "flexbox"
    let has_response = responses
        .iter()
        .any(|r| r.contains("flex") || r.contains("Flexbox") || r.contains("Did you mean"));
    assert!(
        has_response,
        "Should return fuzzy match suggestions: {responses:?}"
    );
}

#[test]
fn mcp_error_component_not_found() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_component","arguments":{"id":"nonexistent/component/id","framework":"react","mode":"light"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    let has_error = responses
        .iter()
        .any(|r| r.contains("error") && r.contains("not found"));
    assert!(
        has_error,
        "Should return component not found error: {responses:?}"
    );
}

#[test]
fn mcp_recommend_components() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"recommend_components","arguments":{"page_types":["landing","blog"]}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Should return recommendations with real component IDs containing "marketing/"
    let has_recommendations = responses.iter().any(|r| r.contains("marketing/"));
    assert!(
        has_recommendations,
        "Should return component recommendations with real IDs: {responses:?}"
    );
}

#[test]
fn mcp_recommend_components_with_real_ids() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"recommend_components","arguments":{"page_types":["landing"]}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Component IDs should contain real category paths like "marketing/"
    let has_real_ids = responses.iter().any(|r| r.contains("marketing/"));
    assert!(
        has_real_ids,
        "Should contain real component IDs with category paths: {responses:?}"
    );
}

#[test]
fn mcp_get_recipe_returns_real_component_ids() {
    let requests = &[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_recipe","arguments":{"pattern":"saas-landing"}}}"#,
    ];

    let responses = run_mcp_session(requests);
    // Recipe should have recommended_components with real IDs
    let has_recommendations = responses
        .iter()
        .any(|r| r.contains("recommended_components") && r.contains("marketing/"));
    assert!(
        has_recommendations,
        "Recipe should contain real component IDs in recommendations: {responses:?}"
    );
}
