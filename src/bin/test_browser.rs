use chromiumoxide::browser::Browser;
use futures::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "ws://43.134.105.109:3001/?token=uJ19BhPcIwonzpD2DNq9SjPlXAIbIYlc",
        "ws://43.134.105.109:3001/chrome?token=uJ19BhPcIwonzpD2DNq9SjPlXAIbIYlc",
    ];

    let mut connected_browser = None;

    for url in urls {
        println!("🔄 Testing: {}", url);
        match tokio::time::timeout(Duration::from_secs(4), Browser::connect(url)).await {
            Ok(Ok((b, mut h))) => {
                println!("✅ SUCCESS with URL: {}", url);
                tokio::spawn(async move {
                    while let Some(event) = h.next().await {
                        if event.is_err() { break; }
                    }
                });
                connected_browser = Some(b);
                break;
            }
            Ok(Err(e)) => println!("❌ Connect error: {}", e),
            Err(_) => println!("⏳ Timeout after 4s"),
        }
    }

    let mut browser = match connected_browser {
        Some(b) => b,
        None => {
            eprintln!("❌ All endpoints failed.");
            std::process::exit(1);
        }
    };

    println!("🌍 Navigating to https://example.com/...");
    let page = browser.new_page("https://example.com").await?;
    
    page.wait_for_navigation().await?;
    let content = page.content().await?;
    
    if content.contains("Example Domain") {
        println!("✅ Semantic check passed: 'Example Domain' found in response.");
    } else {
        println!("⚠️ Warning: Expected text not found in response payload.");
    }

    browser.close().await?;
    Ok(())
}
