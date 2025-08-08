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
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub to: String,
    pub value: String,
    pub nonce: String,
    pub status: String,
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
}

impl App {
    pub fn new(safe_address: Option<String>, rpc_url: Option<String>) -> Self {
        Self {
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
        }
    }

    pub async fn tick(&mut self) {
        // Update data periodically
        if self.last_update.elapsed() > Duration::from_secs(30) {
            self.refresh_data().await;
            self.last_update = Instant::now();
        }
    }

    pub async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<bool> {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
                return Ok(true);
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return Ok(true);
            }
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.state = AppState::Help;
            }
            KeyCode::Char('d') => {
                self.state = AppState::Dashboard;
            }
            KeyCode::Char('t') => {
                self.state = AppState::TransactionList;
                self.refresh_transactions().await;
            }
            KeyCode::Char('r') => {
                self.refresh_data().await;
            }
            KeyCode::Enter => {
                if self.state == AppState::TransactionList && !self.transactions.is_empty() {
                    self.state = AppState::TransactionDetail;
                }
            }
            KeyCode::Backspace | KeyCode::Left => {
                match self.state {
                    AppState::TransactionDetail => self.state = AppState::TransactionList,
                    AppState::Help => self.state = AppState::Dashboard,
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
}
