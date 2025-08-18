use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Row, Table, Wrap,
    },
    Frame,
};

use super::app::{App, AppState};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    draw_header(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Title
    let title = Paragraph::new("🔐 Vito CLI - Safe Wallet Manager")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, header_chunks[0]);

    // Safe address info
    let safe_info = match &app.safe_address {
        Some(addr) => format!("Safe: {}...{}", &addr[..6], &addr[addr.len()-4..]),
        None => "No Safe Address".to_string(),
    };
    let safe_widget = Paragraph::new(safe_info)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(safe_widget, header_chunks[1]);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    // Handle confirmation dialog overlay
    if app.confirmation_dialog.is_some() {
        draw_confirmation_dialog(f, area, app);
        return;
    }

    match app.state {
        AppState::Dashboard => draw_dashboard(f, area, app),
        AppState::TransactionList => draw_transaction_list(f, area, app),
        AppState::TransactionDetail => draw_transaction_detail(f, area, app),
        AppState::TrezorConnect => draw_trezor_connect(f, area, app),
        AppState::Help => draw_help(f, area),
        AppState::CommandInput => draw_command_input(f, area, app),
        AppState::ConfirmationDialog => draw_confirmation_dialog(f, area, app),
        AppState::ProgressView => draw_progress_view(f, area, app),
    }
}

fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Status cards
            Constraint::Min(8),     // Main content area
            Constraint::Length(6),  // Conversation history
        ])
        .split(area);

    // Status cards
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[0]);

    // Safe status card
    let safe_status = if app.safe_address.is_some() {
        "✅ Connected"
    } else {
        "❌ Not Connected"
    };
    let safe_card = Paragraph::new(format!("Safe Wallet\n\n{}", safe_status))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Safe Status"));
    f.render_widget(safe_card, status_chunks[0]);

    // Trezor status card
    let trezor_status = if app.trezor_connected {
        "✅ Connected"
    } else {
        "❌ Not Connected"
    };
    let trezor_card = Paragraph::new(format!("Trezor\n\n{}", trezor_status))
        .style(Style::default().fg(if app.trezor_connected { Color::Green } else { Color::Red }))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Trezor Status"));
    f.render_widget(trezor_card, status_chunks[1]);

    // Transaction count card
    let tx_count = app.transactions.len();
    let tx_card = Paragraph::new(format!("Pending\n\n{} transactions", tx_count))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Transactions"));
    f.render_widget(tx_card, status_chunks[2]);

    // Split main content area between transactions and contextual help
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),  // Recent transactions
            Constraint::Percentage(30),  // Contextual help
        ])
        .split(chunks[1]);

    // Recent transactions preview
    let recent_txs: Vec<ListItem> = app.transactions
        .iter()
        .take(5)
        .map(|tx| {
            ListItem::new(format!("{} → {} ({})",
                &tx.hash[..10],
                &tx.to[..10],
                tx.value
            ))
        })
        .collect();

    let recent_list = List::new(recent_txs)
        .block(Block::default().borders(Borders::ALL).title("Recent Transactions"))
        .style(Style::default().fg(Color::White));
    f.render_widget(recent_list, main_chunks[0]);

    // Contextual help panel
    draw_contextual_help(f, main_chunks[1], app);

    // Conversation history
    draw_conversation_history(f, chunks[2], app);
}

fn draw_transaction_list(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app.transactions
        .iter()
        .enumerate()
        .map(|(i, tx)| {
            let style = if i == app.selected_transaction {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                tx.hash[..10].to_string(),
                tx.to[..10].to_string(),
                tx.value.clone(),
                tx.nonce.clone(),
                tx.status.clone(),
            ]).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(8),
            Constraint::Length(10),
        ]
    )
        .header(
            Row::new(vec!["Hash", "To", "Value", "Nonce", "Status"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .block(Block::default().borders(Borders::ALL).title("Transactions"));

    f.render_widget(table, area);
}

fn draw_transaction_detail(f: &mut Frame, area: Rect, app: &App) {
    if let Some(tx) = app.get_selected_transaction() {
        let details = format!(
            "Hash: {}\n\nTo: {}\n\nValue: {}\n\nNonce: {}\n\nStatus: {}",
            tx.hash, tx.to, tx.value, tx.nonce, tx.status
        );
        
        let detail_widget = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Transaction Details"))
            .wrap(Wrap { trim: true });
        f.render_widget(detail_widget, area);
    }
}

fn draw_trezor_connect(f: &mut Frame, area: Rect, _app: &App) {
    let text = "Connecting to Trezor...\n\nPlease ensure:\n• Trezor is connected\n• Ethereum app is open\n• Device is unlocked";
    let widget = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Trezor Connection"))
        .alignment(Alignment::Center);
    f.render_widget(widget, area);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled("Keyboard Shortcuts", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("Navigation", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled("d", Style::default().fg(Color::Green)), Span::raw(" - Dashboard")]),
        Line::from(vec![Span::styled("t", Style::default().fg(Color::Green)), Span::raw(" - Transaction List")]),
        Line::from(vec![Span::styled("r", Style::default().fg(Color::Green)), Span::raw(" - Refresh Data")]),
        Line::from(vec![Span::styled("h/F1", Style::default().fg(Color::Green)), Span::raw(" - Help")]),
        Line::from(vec![Span::styled("↑/↓", Style::default().fg(Color::Green)), Span::raw(" - Navigate Lists")]),
        Line::from(vec![Span::styled("Enter", Style::default().fg(Color::Green)), Span::raw(" - Select/View Details")]),
        Line::from(vec![Span::styled("Backspace/←", Style::default().fg(Color::Green)), Span::raw(" - Go Back")]),
        Line::from(""),
        Line::from(vec![Span::styled("Conversational Commands", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(":", Style::default().fg(Color::Yellow)), Span::raw(" - Enter command mode")]),
        Line::from(vec![Span::styled("/", Style::default().fg(Color::Yellow)), Span::raw(" - Alternative command mode")]),
        Line::from(""),
        Line::from(vec![Span::styled("Natural Language Examples:", Style::default().fg(Color::Magenta))]),
        Line::from(vec![Span::raw("  • 'show transactions' - Display transaction list")]),
        Line::from(vec![Span::raw("  • 'connect trezor' - Connect hardware wallet")]),
        Line::from(vec![Span::raw("  • 'refresh data' - Update all data")]),
        Line::from(vec![Span::raw("  • 'sign transaction' - Sign selected transaction")]),
        Line::from(vec![Span::raw("  • 'help' - Show this help")]),
        Line::from(""),
        Line::from(vec![Span::styled("q/Esc", Style::default().fg(Color::Red)), Span::raw(" - Quit")]),
        Line::from(vec![Span::styled("Ctrl+C", Style::default().fg(Color::Red)), Span::raw(" - Force Quit")]),
    ];

    let help_widget = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help - Enhanced with Conversational Interface"))
        .alignment(Alignment::Left);
    f.render_widget(help_widget, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // Status message
    let status = Paragraph::new(app.status_message.clone())
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, footer_chunks[0]);

    // Navigation hint - updated for conversational mode
    let nav_hint = match app.state {
        AppState::CommandInput => "Type command, Enter to execute, Esc to cancel",
        _ => "Press ':' for commands, 'h' for help, 'q' to quit",
    };
    let nav_widget = Paragraph::new(nav_hint)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(nav_widget, footer_chunks[1]);
}

fn draw_contextual_help(f: &mut Frame, area: Rect, app: &App) {
    let help_items: Vec<ListItem> = app.contextual_help
        .iter()
        .map(|help| ListItem::new(help.clone()))
        .collect();

    let help_list = List::new(help_items)
        .block(Block::default().borders(Borders::ALL).title("Quick Help"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(help_list, area);
}

fn draw_conversation_history(f: &mut Frame, area: Rect, app: &App) {
    let history_items: Vec<ListItem> = app.conversation_history
        .iter()
        .rev()
        .take(3)
        .rev()
        .map(|msg| ListItem::new(msg.clone()))
        .collect();

    let history_list = List::new(history_items)
        .block(Block::default().borders(Borders::ALL).title("Recent Actions"))
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(history_list, area);
}

fn draw_command_input(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Input field
            Constraint::Min(0),     // Suggestions
        ])
        .split(area);

    // Command input field
    let input_text = format!("{}_", app.command_input.input);
    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Command Input"));
    f.render_widget(input_widget, chunks[0]);

    // Suggestions
    if !app.command_input.suggestions.is_empty() {
        let suggestions: Vec<ListItem> = app.command_input.suggestions
            .iter()
            .map(|suggestion| ListItem::new(suggestion.clone()))
            .collect();

        let suggestions_list = List::new(suggestions)
            .block(Block::default().borders(Borders::ALL).title("Suggestions"))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(suggestions_list, chunks[1]);
    } else {
        // Show command examples when no suggestions
        let examples = vec![
            "show transactions - Display transaction list",
            "connect trezor - Connect hardware wallet",
            "refresh data - Update all data",
            "sign transaction - Sign selected transaction",
            "help - Show help information",
        ];

        let example_items: Vec<ListItem> = examples
            .iter()
            .map(|example| ListItem::new(*example))
            .collect();

        let examples_list = List::new(example_items)
            .block(Block::default().borders(Borders::ALL).title("Command Examples"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(examples_list, chunks[1]);
    }
}

fn draw_confirmation_dialog(f: &mut Frame, area: Rect, app: &App) {
    if let Some(dialog) = &app.confirmation_dialog {
        // Create a centered popup
        let popup_area = centered_rect(60, 20, area);

        // Clear the background
        f.render_widget(
            Block::default()
                .style(Style::default().bg(Color::Black))
                .borders(Borders::ALL),
            popup_area,
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),     // Message
                Constraint::Length(3),  // Buttons
            ])
            .margin(1)
            .split(popup_area);

        // Dialog message
        let message = Paragraph::new(dialog.message.clone())
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(message, chunks[0]);

        // Buttons
        let buttons = Paragraph::new("Press 'y' for Yes, 'n' for No")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(buttons, chunks[1]);
    }
}

fn draw_progress_view(f: &mut Frame, area: Rect, app: &App) {
    let progress_widget = Paragraph::new(app.progress_message.clone())
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Progress"));
    f.render_widget(progress_widget, area);
}

// Helper function to create centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
