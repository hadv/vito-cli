# Vito CLI TUI Demo

## What the TUI looks like

When you run `vito tui`, you'll see a modern terminal interface like this:

```
┌─ 🔐 Vito CLI - Safe Wallet Manager ─┐┌─ Safe: 0x1234...5678 ─┐
│                                      ││                       │
└──────────────────────────────────────┘└───────────────────────┘

┌─ Safe Status ─┐┌─ Trezor Status ─┐┌─ Transactions ─┐
│ Safe Wallet   ││ Trezor          ││ Pending        │
│               ││                 ││                │
│ ✅ Connected  ││ ❌ Not Connected││ 3 transactions │
└───────────────┘└─────────────────┘└────────────────┘

┌─ Recent Transactions ──────────────────────────────────────┐
│ 0x1234abcd → 0x5678efgh (1.5 ETH)                         │
│ 0x9876dcba → 0x4321fedc (0.1 ETH)                         │
│ 0xabcd1234 → 0xefgh5678 (2.0 ETH)                         │
│                                                            │
│                                                            │
└────────────────────────────────────────────────────────────┘

┌─ Status: Welcome to Vito TUI ─────────┐┌─ Press 'h' for help ─┐
│                                       ││ 'q' to quit          │
└───────────────────────────────────────┘└──────────────────────┘
```

## Navigation

### Dashboard View (Press 'd')
- Overview of Safe wallet connection status
- Trezor hardware wallet connection indicator
- Count of pending transactions
- Preview of recent transactions

### Transaction List View (Press 't')
```
┌─ Transactions ─────────────────────────────────────────────────┐
│ Hash       │ To         │ Value      │ Nonce │ Status          │
├────────────┼────────────┼────────────┼───────┼─────────────────┤
│ 0x1234...  │ 0xabcd...  │ 1.5 ETH    │ 42    │ Pending         │
│ 0x5678...  │ 0xefgh...  │ 0.1 ETH    │ 43    │ Pending         │
│ 0x9abc...  │ 0x1234...  │ 2.0 ETH    │ 44    │ Pending         │
└────────────┴────────────┴────────────┴───────┴─────────────────┘
```

Use ↑/↓ arrows to navigate, Enter to view details.

### Transaction Detail View (Press Enter on selected transaction)
```
┌─ Transaction Details ──────────────────────────────────────────┐
│ Hash: 0x1234567890abcdef1234567890abcdef1234567890abcdef123456 │
│                                                                │
│ To: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef   │
│                                                                │
│ Value: 1.5 ETH                                                 │
│                                                                │
│ Nonce: 42                                                      │
│                                                                │
│ Status: Pending                                                │
└────────────────────────────────────────────────────────────────┘
```

Press Backspace or ← to go back to the transaction list.

### Help View (Press 'h' or F1)
```
┌─ Help ─────────────────────────────────────────────────────────┐
│ Keyboard Shortcuts                                             │
│                                                                │
│ d - Dashboard                                                  │
│ t - Transaction List                                           │
│ r - Refresh Data                                               │
│ h/F1 - Help                                                    │
│ ↑/↓ - Navigate Lists                                           │
│ Enter - Select/View Details                                    │
│ Backspace/← - Go Back                                          │
│ q/Esc - Quit                                                   │
│ Ctrl+C - Force Quit                                            │
└────────────────────────────────────────────────────────────────┘
```

## Features

### Real-time Updates
- The TUI automatically refreshes transaction data every 30 seconds
- Press 'r' to manually refresh at any time
- Status messages show when data is being updated

### Keyboard Navigation
- Fully keyboard-driven interface
- Intuitive navigation with arrow keys
- Quick access to all features via hotkeys

### Responsive Design
- Adapts to different terminal sizes
- Clean, modern interface with borders and colors
- Clear visual hierarchy and status indicators

### Integration with Existing Commands
- Uses the same backend logic as the CLI commands
- Seamless integration with Trezor hardware wallet
- Compatible with all existing Safe wallet operations

## Getting Started

1. **Install Vito CLI** (see main README for installation instructions)

2. **Set up environment variables**:
   ```bash
   export RPC_URL="https://mainnet.infura.io/v3/YOUR_PROJECT_ID"
   ```

3. **Start the TUI**:
   ```bash
   # Basic TUI mode
   vito tui
   
   # With specific Safe address
   vito --safe 0x1234567890123456789012345678901234567890 tui
   ```

4. **Navigate the interface**:
   - Press 'h' for help
   - Press 'd' for dashboard
   - Press 't' to view transactions
   - Press 'q' to quit

The TUI provides a modern, user-friendly way to interact with your Safe wallet without leaving the terminal!
