use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Dashboard,
    TransactionList,
    TransactionDetail,
    TrezorConnect,
    Help,
    // New conversational states
    CommandInput,
    ConfirmationDialog,
    ProgressView,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub to: String,
    pub value: String,
    pub nonce: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct CommandInput {
    pub input: String,
    pub cursor_position: usize,
    pub suggestions: Vec<String>,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
}

impl CommandInput {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            suggestions: Vec::new(),
            history: Vec::new(),
            history_index: None,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
        self.update_suggestions();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
            self.update_suggestions();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
        self.suggestions.clear();
        self.history_index = None;
    }

    pub fn submit(&mut self) -> String {
        let command = self.input.clone();
        if !command.trim().is_empty() {
            self.history.push(command.clone());
        }
        self.clear();
        command
    }

    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        match self.history_index {
            None => {
                self.history_index = Some(self.history.len() - 1);
                self.input = self.history[self.history.len() - 1].clone();
            }
            Some(index) if index > 0 => {
                self.history_index = Some(index - 1);
                self.input = self.history[index - 1].clone();
            }
            _ => {}
        }
        self.cursor_position = self.input.len();
    }

    pub fn history_down(&mut self) {
        match self.history_index {
            Some(index) if index < self.history.len() - 1 => {
                self.history_index = Some(index + 1);
                self.input = self.history[index + 1].clone();
            }
            Some(_) => {
                self.history_index = None;
                self.input.clear();
            }
            None => {}
        }
        self.cursor_position = self.input.len();
    }

    fn update_suggestions(&mut self) {
        self.suggestions.clear();
        if self.input.is_empty() {
            return;
        }

        let input_lower = self.input.to_lowercase();
        let mut suggestions = Vec::new();

        // Command suggestions based on input
        let commands = [
            ("show transactions", "Display all pending transactions"),
            ("connect trezor", "Connect to Trezor hardware wallet"),
            ("refresh data", "Refresh all data from the blockchain"),
            ("sign transaction", "Sign the selected transaction"),
            ("help", "Show help information"),
            ("quit", "Exit the application"),
            ("dashboard", "Go to dashboard view"),
            ("list transactions", "Show transaction list"),
        ];

        for (cmd, desc) in commands.iter() {
            if cmd.starts_with(&input_lower) {
                suggestions.push(format!("{} - {}", cmd, desc));
            }
        }

        self.suggestions = suggestions;
    }
}

#[derive(Debug, Clone)]
pub struct ConfirmationDialog {
    pub message: String,
    pub confirmed: Option<bool>,
}

impl ConfirmationDialog {
    pub fn new(message: String) -> Self {
        Self {
            message,
            confirmed: None,
        }
    }
}

pub struct App {
    pub state: AppState,
    pub safe_address: Option<String>,
    pub rpc_url: Option<String>,
    pub transactions: Vec<Transaction>,
    pub selected_transaction: usize,
    pub trezor_connected: bool,
    pub trezor_address: Option<String>,
    pub last_update: Instant,
    pub status_message: String,
    pub should_quit: bool,
    // New conversational elements
    pub command_input: CommandInput,
    pub confirmation_dialog: Option<ConfirmationDialog>,
    pub progress_message: String,
    pub contextual_help: Vec<String>,
    pub conversation_history: Vec<String>,
}

impl App {
    pub fn new(safe_address: Option<String>, rpc_url: Option<String>) -> Self {
        let mut app = Self {
            state: AppState::Dashboard,
            safe_address,
            rpc_url,
            transactions: Vec::new(),
            selected_transaction: 0,
            trezor_connected: false,
            trezor_address: None,
            last_update: Instant::now(),
            status_message: "Welcome to Vito TUI".to_string(),
            should_quit: false,
            command_input: CommandInput::new(),
            confirmation_dialog: None,
            progress_message: String::new(),
            contextual_help: Vec::new(),
            conversation_history: Vec::new(),
        };
        app.update_contextual_help();
        app
    }

    pub async fn tick(&mut self) {
        // Update data periodically
        if self.last_update.elapsed() > Duration::from_secs(30) {
            self.refresh_data().await;
            self.last_update = Instant::now();
        }

        // Update real-time status
        self.update_connection_status().await;
        self.check_pending_operations().await;
    }

    async fn update_connection_status(&mut self) {
        // Mock implementation - in real app, check actual connection
        // This would check Trezor connection, RPC connection, etc.
    }

    async fn check_pending_operations(&mut self) {
        // Mock implementation - in real app, check for pending operations
        // This would check for pending transactions, background tasks, etc.
    }

    pub async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<bool> {
        // Handle confirmation dialog first
        if let Some(ref mut dialog) = self.confirmation_dialog {
            match key_event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    dialog.confirmed = Some(true);
                    self.handle_confirmation_result().await;
                    return Ok(false);
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    dialog.confirmed = Some(false);
                    self.handle_confirmation_result().await;
                    return Ok(false);
                }
                _ => return Ok(false),
            }
        }

        // Handle command input mode
        if self.state == AppState::CommandInput {
            return self.handle_command_input_key(key_event).await;
        }

        // Global shortcuts
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.state == AppState::CommandInput {
                    self.state = AppState::Dashboard;
                    self.update_contextual_help();
                } else {
                    self.should_quit = true;
                    return Ok(true);
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return Ok(true);
            }
            KeyCode::Char(':') => {
                // Enter command mode (like vim)
                self.state = AppState::CommandInput;
                self.command_input.clear();
                self.update_contextual_help();
            }
            KeyCode::Char('/') => {
                // Alternative command mode entry
                self.state = AppState::CommandInput;
                self.command_input.clear();
                self.update_contextual_help();
            }
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.state = AppState::Help;
                self.update_contextual_help();
            }
            KeyCode::Char('d') => {
                self.state = AppState::Dashboard;
                self.update_contextual_help();
            }
            KeyCode::Char('t') => {
                self.state = AppState::TransactionList;
                self.refresh_transactions().await;
                self.update_contextual_help();
            }
            KeyCode::Char('r') => {
                self.refresh_data().await;
            }
            KeyCode::Enter => {
                if self.state == AppState::TransactionList && !self.transactions.is_empty() {
                    self.state = AppState::TransactionDetail;
                    self.update_contextual_help();
                }
            }
            KeyCode::Backspace | KeyCode::Left => {
                match self.state {
                    AppState::TransactionDetail => {
                        self.state = AppState::TransactionList;
                        self.update_contextual_help();
                    }
                    AppState::Help => {
                        self.state = AppState::Dashboard;
                        self.update_contextual_help();
                    }
                    _ => {}
                }
            }
            KeyCode::Up => {
                if self.state == AppState::TransactionList && self.selected_transaction > 0 {
                    self.selected_transaction -= 1;
                }
            }
            KeyCode::Down => {
                if self.state == AppState::TransactionList
                    && self.selected_transaction < self.transactions.len().saturating_sub(1) {
                    self.selected_transaction += 1;
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn refresh_data(&mut self) {
        self.status_message = "Refreshing data...".to_string();
        self.refresh_transactions().await;
        // Add other data refresh logic here
        self.status_message = "Data refreshed".to_string();
    }

    async fn refresh_transactions(&mut self) {
        if self.safe_address.is_none() {
            self.status_message = "No Safe address configured".to_string();
            return;
        }

        // Mock data for now - in real implementation, this would call the tx module
        self.transactions = vec![
            Transaction {
                hash: "0x1234...".to_string(),
                to: "0xabcd...".to_string(),
                value: "1.5 ETH".to_string(),
                nonce: "42".to_string(),
                status: "Pending".to_string(),
            },
            Transaction {
                hash: "0x5678...".to_string(),
                to: "0xefgh...".to_string(),
                value: "0.1 ETH".to_string(),
                nonce: "43".to_string(),
                status: "Pending".to_string(),
            },
        ];
        
        self.status_message = format!("Loaded {} transactions", self.transactions.len());
    }

    pub fn get_selected_transaction(&self) -> Option<&Transaction> {
        self.transactions.get(self.selected_transaction)
    }

    async fn handle_command_input_key(&mut self, key_event: KeyEvent) -> Result<bool> {
        match key_event.code {
            KeyCode::Char(c) => {
                self.command_input.insert_char(c);
            }
            KeyCode::Backspace => {
                self.command_input.delete_char();
            }
            KeyCode::Left => {
                self.command_input.move_cursor_left();
            }
            KeyCode::Right => {
                self.command_input.move_cursor_right();
            }
            KeyCode::Up => {
                self.command_input.history_up();
            }
            KeyCode::Down => {
                self.command_input.history_down();
            }
            KeyCode::Enter => {
                let command = self.command_input.submit();
                self.process_natural_language_command(command).await;
                self.state = AppState::Dashboard;
                self.update_contextual_help();
            }
            KeyCode::Esc => {
                self.state = AppState::Dashboard;
                self.update_contextual_help();
            }
            _ => {}
        }
        Ok(false)
    }

    async fn process_natural_language_command(&mut self, command: String) {
        let command_lower = command.to_lowercase();
        self.conversation_history.push(format!("> {}", command));

        match command_lower.as_str() {
            cmd if cmd.contains("show") && cmd.contains("transaction") => {
                self.state = AppState::TransactionList;
                self.refresh_transactions().await;
                self.conversation_history.push("📋 Showing transaction list".to_string());
                self.status_message = "Switched to transaction list view".to_string();
            }
            cmd if cmd.contains("connect") && cmd.contains("trezor") => {
                self.state = AppState::TrezorConnect;
                self.conversation_history.push("🔗 Connecting to Trezor...".to_string());
                self.status_message = "Attempting to connect to Trezor".to_string();
            }
            cmd if cmd.contains("refresh") || cmd.contains("reload") => {
                self.refresh_data().await;
                self.conversation_history.push("🔄 Data refreshed".to_string());
            }
            cmd if cmd.contains("sign") && cmd.contains("transaction") => {
                if !self.transactions.is_empty() {
                    self.confirmation_dialog = Some(ConfirmationDialog::new(
                        "Do you want to sign the selected transaction?".to_string()
                    ));
                    self.conversation_history.push("⚠️  Requesting confirmation for transaction signing".to_string());
                } else {
                    self.conversation_history.push("❌ No transactions available to sign".to_string());
                    self.status_message = "No transactions available".to_string();
                }
            }
            cmd if cmd.contains("help") => {
                self.state = AppState::Help;
                self.conversation_history.push("❓ Showing help".to_string());
                self.status_message = "Help view activated".to_string();
            }
            cmd if cmd.contains("dashboard") || cmd.contains("home") => {
                self.state = AppState::Dashboard;
                self.conversation_history.push("🏠 Returning to dashboard".to_string());
                self.status_message = "Dashboard view activated".to_string();
            }
            cmd if cmd.contains("quit") || cmd.contains("exit") => {
                self.confirmation_dialog = Some(ConfirmationDialog::new(
                    "Are you sure you want to quit?".to_string()
                ));
                self.conversation_history.push("⚠️  Requesting confirmation to quit".to_string());
            }
            _ => {
                self.conversation_history.push(format!("❓ Unknown command: '{}'. Try 'help' for available commands.", command));
                self.status_message = format!("Unknown command: '{}'", command);
            }
        }
        self.update_contextual_help();
    }

    async fn handle_confirmation_result(&mut self) {
        if let Some(dialog) = &self.confirmation_dialog {
            match dialog.confirmed {
                Some(true) => {
                    if dialog.message.contains("quit") {
                        self.should_quit = true;
                    } else if dialog.message.contains("sign") {
                        // Handle transaction signing
                        self.conversation_history.push("✅ Transaction signing confirmed".to_string());
                        self.status_message = "Transaction signing initiated".to_string();
                        // In real implementation, call signing logic here
                    }
                }
                Some(false) => {
                    self.conversation_history.push("❌ Action cancelled".to_string());
                    self.status_message = "Action cancelled".to_string();
                }
                None => {}
            }
        }
        self.confirmation_dialog = None;
    }

    pub fn update_contextual_help(&mut self) {
        self.contextual_help.clear();

        match self.state {
            AppState::Dashboard => {
                self.contextual_help = vec![
                    "Press ':' or '/' to enter command mode".to_string(),
                    "Press 't' to view transactions".to_string(),
                    "Press 'c' to connect Trezor".to_string(),
                    "Press 'r' to refresh data".to_string(),
                    "Press 'h' for help".to_string(),
                ];
            }
            AppState::TransactionList => {
                self.contextual_help = vec![
                    "Use ↑/↓ to navigate transactions".to_string(),
                    "Press Enter to view details".to_string(),
                    "Press ':' for command mode".to_string(),
                    "Say 'sign transaction' to sign selected".to_string(),
                    "Press Backspace to go back".to_string(),
                ];
            }
            AppState::TransactionDetail => {
                self.contextual_help = vec![
                    "Press Backspace to return to list".to_string(),
                    "Press ':' for command mode".to_string(),
                    "Say 'sign this transaction' to sign".to_string(),
                ];
            }
            AppState::CommandInput => {
                self.contextual_help = vec![
                    "Type natural language commands".to_string(),
                    "Examples: 'show transactions', 'connect trezor'".to_string(),
                    "Use ↑/↓ for command history".to_string(),
                    "Press Enter to execute, Esc to cancel".to_string(),
                ];
            }
            AppState::Help => {
                self.contextual_help = vec![
                    "Press any key to return to previous view".to_string(),
                    "Press ':' for command mode".to_string(),
                ];
            }
            _ => {
                self.contextual_help = vec![
                    "Press ':' for command mode".to_string(),
                    "Press 'h' for help".to_string(),
                ];
            }
        }
    }
}
