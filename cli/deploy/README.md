# Deploy Fortune
To deploy the fortune cli:
1. Copy the fortune files to the appropriate folder

| OS      | Location                                                    |
|---------|-------------------------------------------------------------|
| Windows | `C:\Users\<username>\AppData\Roaming\psobolik\fortune\data` |
| Linux   | `~/.local/share/fortune`                                    |
| macOS   | `~/Library/Application Support/home.psobolik.fortune`       |

1. Build the project `$ cargo build --release`
2. Copy the executable, `fortune` or `fortune.exe`, from `./target/release` to a folder in your path. 
