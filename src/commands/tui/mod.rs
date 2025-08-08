use anyhow::Result;
use std::env;
use ethers::types::Address;
use std::str::FromStr;

mod app;
mod events;
mod ui;

use app::App;
use events::{Event, EventHandler};

/// Execute the TUI mode
pub async fn execute(safe: Option<String>, rpc: Option<String>) -> Result<()> {
    // Validate Safe address if provided
    let _safe_address = match &safe {
        Some(addr) => {
            Some(Address::from_str(addr)
                .map_err(|_| anyhow::anyhow!("Invalid Safe wallet address format"))?)
        },
        None => None,
    };

    // Get RPC URL from parameter or environment
    let rpc_url = rpc.or_else(|| env::var("RPC_URL").ok());

    // Initialize terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(safe, rpc_url);

    // Create event handler
    let event_handler = EventHandler::new(250);

    // Main application loop
    let result = run_app(&mut terminal, &mut app, event_handler).await;

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut App,
    mut event_handler: EventHandler,
) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        match event_handler.next().await? {
            Event::Tick => {
                app.tick().await;
            }
            Event::Key(key_event) => {
                if app.handle_key_event(key_event).await? {
                    break;
                }
            }
            Event::Mouse(_) => {
                // Handle mouse events if needed
            }
            Event::Resize(_, _) => {
                // Terminal was resized, will be handled automatically
            }
        }
    }
    Ok(())
}
