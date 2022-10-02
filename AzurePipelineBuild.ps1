$rust_installer_url = 'https://win.rustup.rs/x86_64'

# Download Rust installer
if ($false) {
	# For Windows
	$fileName = $env:TEMP + (Split-Path -Path $rust_installer_url -Leaf)
	Write-Host "Executable File Name: $fileName"
	Invoke-RestMethod -Uri $rust_installer_url -OutFile $fileName

	# Run Rust installer
	Invoke-Item $fileName
} else {
	# For Unix (Linux / Mac)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}

# Run Rust updater to assure installation succeeded.
rustup update

# Install Trunk
cargo install trunk

# Set build target
rustup target add wasm32-unknown-unknown

# Run tests
$test_results = cargo test | Out-String

if ($test_results.Contains("test result: FAILED")) {
	throw "Tests Failed"
}

# Change directory to webapp
cd webapp

# Build
trunk build
