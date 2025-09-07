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
    pub aliases: std::collections::HashMap<String, String>,
}

impl CommandInput {
    pub fn new() -> Self {
        let mut aliases = std::collections::HashMap::new();

        // Default aliases
        aliases.insert("tx".to_string(), "show transactions".to_string());
        aliases.insert("ls".to_string(), "show transactions".to_string());
        aliases.insert("list".to_string(), "show transactions".to_string());
        aliases.insert("conn".to_string(), "connect trezor".to_string());
        aliases.insert("connect".to_string(), "connect trezor".to_string());
        aliases.insert("ref".to_string(), "refresh data".to_string());
        aliases.insert("reload".to_string(), "refresh data".to_string());
        aliases.insert("sign".to_string(), "sign transaction".to_string());
        aliases.insert("s".to_string(), "sign transaction".to_string());
        aliases.insert("home".to_string(), "dashboard".to_string());
        aliases.insert("main".to_string(), "dashboard".to_string());
        aliases.insert("exit".to_string(), "quit".to_string());
        aliases.insert("q".to_string(), "quit".to_string());
        aliases.insert("?".to_string(), "help".to_string());

        Self {
            input: String::new(),
            cursor_position: 0,
            suggestions: Vec::new(),
            history: Vec::new(),
            history_index: None,
            aliases,
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
        let mut command = self.input.clone();

        // Check for aliases first
        if let Some(expanded) = self.aliases.get(&command.trim().to_lowercase()) {
            command = expanded.clone();
        }

        if !command.trim().is_empty() {
            self.history.push(command.clone());
        }
        self.clear();
        command
    }

    pub fn add_alias(&mut self, alias: String, command: String) {
        self.aliases.insert(alias.to_lowercase(), command);
    }

    pub fn remove_alias(&mut self, alias: &str) {
        self.aliases.remove(&alias.to_lowercase());
    }

    pub fn list_aliases(&self) -> Vec<(String, String)> {
        self.aliases.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
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

        // Extended command suggestions with more natural language patterns
        let commands = [
            // Transaction commands
            ("show transactions", "Display all pending transactions"),
            ("list transactions", "Show transaction list"),
            ("view transactions", "Browse transaction history"),
            ("get transactions", "Fetch transaction data"),
            ("display transactions", "Show transaction details"),
            ("transactions", "Go to transaction view"),

            // Connection commands
            ("connect trezor", "Connect to Trezor hardware wallet"),
            ("connect wallet", "Connect hardware wallet"),
            ("connect device", "Connect to hardware device"),
            ("setup trezor", "Initialize Trezor connection"),
            ("link trezor", "Establish Trezor connection"),

            // Data refresh commands
            ("refresh data", "Refresh all data from the blockchain"),
            ("update data", "Update blockchain information"),
            ("reload data", "Reload all information"),
            ("sync data", "Synchronize with blockchain"),
            ("fetch latest", "Get latest blockchain data"),

            // Transaction signing
            ("sign transaction", "Sign the selected transaction"),
            ("sign tx", "Sign current transaction"),
            ("approve transaction", "Approve and sign transaction"),
            ("confirm transaction", "Confirm transaction signing"),

            // Navigation commands
            ("dashboard", "Go to dashboard view"),
            ("home", "Return to main dashboard"),
            ("main menu", "Go to main interface"),
            ("overview", "Show dashboard overview"),

            // Help and info
            ("help", "Show help information"),
            ("info", "Display application information"),
            ("guide", "Show user guide"),
            ("commands", "List available commands"),
            ("shortcuts", "Show keyboard shortcuts"),

            // Exit commands
            ("quit", "Exit the application"),
            ("exit", "Close the application"),
            ("close", "Terminate the program"),
            ("bye", "Exit gracefully"),

            // Status commands
            ("status", "Show current status"),
            ("check status", "Display connection status"),
            ("health check", "Verify system health"),
        ];

        // Check full commands
        for (cmd, desc) in commands.iter() {
            if cmd.starts_with(&input_lower) {
                suggestions.push(format!("{} - {}", cmd, desc));
            }
        }

        // Check aliases
        for (alias, full_cmd) in self.aliases.iter() {
            if alias.starts_with(&input_lower) {
                suggestions.push(format!("{} → {} (alias)", alias, full_cmd));
            }
        }

        // Fuzzy matching for partial words
        if suggestions.is_empty() {
            for (cmd, desc) in commands.iter() {
                if cmd.contains(&input_lower) {
                    suggestions.push(format!("{} - {} (fuzzy match)", cmd, desc));
                }
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

        // Handle alias management commands first
        if command_lower.starts_with("alias ") {
            self.handle_alias_command(&command_lower).await;
            return;
        }

        match command_lower.as_str() {
            // Transaction viewing commands (expanded patterns)
            cmd if cmd.contains("show") && cmd.contains("transaction") ||
                   cmd.contains("list") && cmd.contains("transaction") ||
                   cmd.contains("view") && cmd.contains("transaction") ||
                   cmd.contains("display") && cmd.contains("transaction") ||
                   cmd.contains("get") && cmd.contains("transaction") ||
                   cmd == "transactions" => {
                self.state = AppState::TransactionList;
                self.refresh_transactions().await;
                self.conversation_history.push("📋 Showing transaction list".to_string());
                self.status_message = "Switched to transaction list view".to_string();
            }

            // Connection commands (expanded patterns)
            cmd if cmd.contains("connect") && (cmd.contains("trezor") || cmd.contains("wallet") || cmd.contains("device")) ||
                   cmd.contains("setup") && cmd.contains("trezor") ||
                   cmd.contains("link") && cmd.contains("trezor") => {
                self.state = AppState::TrezorConnect;
                self.conversation_history.push("🔗 Connecting to Trezor...".to_string());
                self.status_message = "Attempting to connect to Trezor".to_string();
            }

            // Data refresh commands (expanded patterns)
            cmd if cmd.contains("refresh") || cmd.contains("reload") || cmd.contains("update") ||
                   cmd.contains("sync") || cmd.contains("fetch") => {
                self.refresh_data().await;
                self.conversation_history.push("🔄 Data refreshed".to_string());
            }

            // Transaction signing commands (expanded patterns)
            cmd if (cmd.contains("sign") || cmd.contains("approve") || cmd.contains("confirm")) &&
                   (cmd.contains("transaction") || cmd.contains("tx")) => {
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

            // Help commands (expanded patterns)
            cmd if cmd.contains("help") || cmd.contains("info") || cmd.contains("guide") ||
                   cmd.contains("commands") || cmd.contains("shortcuts") => {
                self.state = AppState::Help;
                self.conversation_history.push("❓ Showing help".to_string());
                self.status_message = "Help view activated".to_string();
            }

            // Navigation commands (expanded patterns)
            cmd if cmd.contains("dashboard") || cmd.contains("home") || cmd.contains("main") ||
                   cmd.contains("overview") => {
                self.state = AppState::Dashboard;
                self.conversation_history.push("🏠 Returning to dashboard".to_string());
                self.status_message = "Dashboard view activated".to_string();
            }

            // Status commands (new)
            cmd if cmd.contains("status") || cmd.contains("health") => {
                self.show_status_info().await;
            }

            // Exit commands (expanded patterns)
            cmd if cmd.contains("quit") || cmd.contains("exit") || cmd.contains("close") ||
                   cmd.contains("bye") => {
                self.confirmation_dialog = Some(ConfirmationDialog::new(
                    "Are you sure you want to quit?".to_string()
                ));
                self.conversation_history.push("⚠️  Requesting confirmation to quit".to_string());
            }

            _ => {
                self.conversation_history.push(format!("❓ Unknown command: '{}'. Try 'help' for available commands or 'alias list' to see shortcuts.", command));
                self.status_message = format!("Unknown command: '{}'", command);
            }
        }
        self.update_contextual_help();
    }

    async fn handle_alias_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["alias", "list"] => {
                let aliases = self.command_input.list_aliases();
                if aliases.is_empty() {
                    self.conversation_history.push("📝 No custom aliases defined".to_string());
                } else {
                    self.conversation_history.push("📝 Available aliases:".to_string());
                    for (alias, cmd) in aliases.iter().take(5) {
                        self.conversation_history.push(format!("  {} → {}", alias, cmd));
                    }
                }
                self.status_message = format!("Listed {} aliases", aliases.len());
            }
            ["alias", "add", alias, command @ ..] if !alias.is_empty() && !command.is_empty() => {
                let full_command = command.join(" ");
                self.command_input.add_alias(alias.to_string(), full_command.clone());
                self.conversation_history.push(format!("✅ Added alias: {} → {}", alias, full_command));
                self.status_message = format!("Added alias '{}'", alias);
            }
            ["alias", "remove", alias] if !alias.is_empty() => {
                self.command_input.remove_alias(alias);
                self.conversation_history.push(format!("🗑️  Removed alias: {}", alias));
                self.status_message = format!("Removed alias '{}'", alias);
            }
            _ => {
                self.conversation_history.push("❓ Alias usage: 'alias list', 'alias add <name> <command>', 'alias remove <name>'".to_string());
                self.status_message = "Invalid alias command syntax".to_string();
            }
        }
    }

    async fn show_status_info(&mut self) {
        let safe_status = if self.safe_address.is_some() { "Connected" } else { "Not Connected" };
        let trezor_status = if self.trezor_connected { "Connected" } else { "Not Connected" };
        let tx_count = self.transactions.len();

        self.conversation_history.push("📊 System Status:".to_string());
        self.conversation_history.push(format!("  Safe: {}", safe_status));
        self.conversation_history.push(format!("  Trezor: {}", trezor_status));
        self.conversation_history.push(format!("  Transactions: {}", tx_count));

        self.status_message = "Status information displayed".to_string();
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
                    "Type natural language commands or aliases".to_string(),
                    "Examples: 'show transactions', 'tx', 'conn'".to_string(),
                    "Alias commands: 'alias list', 'alias add'".to_string(),
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
