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
    match app.state {
        AppState::Dashboard => draw_dashboard(f, area, app),
        AppState::TransactionList => draw_transaction_list(f, area, app),
        AppState::TransactionDetail => draw_transaction_detail(f, area, app),
        AppState::TrezorConnect => draw_trezor_connect(f, area, app),
        AppState::Help => draw_help(f, area),
    }
}

fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Status cards
            Constraint::Min(0),     // Recent transactions
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
    f.render_widget(recent_list, chunks[1]);
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
        Line::from(vec![Span::styled("d", Style::default().fg(Color::Green)), Span::raw(" - Dashboard")]),
        Line::from(vec![Span::styled("t", Style::default().fg(Color::Green)), Span::raw(" - Transaction List")]),
        Line::from(vec![Span::styled("r", Style::default().fg(Color::Green)), Span::raw(" - Refresh Data")]),
        Line::from(vec![Span::styled("h/F1", Style::default().fg(Color::Green)), Span::raw(" - Help")]),
        Line::from(vec![Span::styled("↑/↓", Style::default().fg(Color::Green)), Span::raw(" - Navigate Lists")]),
        Line::from(vec![Span::styled("Enter", Style::default().fg(Color::Green)), Span::raw(" - Select/View Details")]),
        Line::from(vec![Span::styled("Backspace/←", Style::default().fg(Color::Green)), Span::raw(" - Go Back")]),
        Line::from(vec![Span::styled("q/Esc", Style::default().fg(Color::Red)), Span::raw(" - Quit")]),
        Line::from(vec![Span::styled("Ctrl+C", Style::default().fg(Color::Red)), Span::raw(" - Force Quit")]),
    ];

    let help_widget = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
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

    // Navigation hint
    let nav_hint = Paragraph::new("Press 'h' for help, 'q' to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(nav_hint, footer_chunks[1]);
}
