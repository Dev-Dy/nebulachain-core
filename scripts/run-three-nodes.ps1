# Create node data directories
New-Item -ItemType Directory -Force -Path C:\nebula_node_a | Out-Null
New-Item -ItemType Directory -Force -Path C:\nebula_node_b | Out-Null
New-Item -ItemType Directory -Force -Path C:\nebula_node_c | Out-Null

Write-Host "Starting NebulaChain 3-node network..."

# Node A
Start-Process powershell -ArgumentList "cargo run -- --db C:\nebula_node_a --p2p 127.0.0.1:9000 --rpc 127.0.0.1:18000 --bootstrap ''"

# Node B
Start-Process powershell -ArgumentList "cargo run -- --db C:\nebula_node_b --p2p 127.0.0.1:9001 --rpc 127.0.0.1:18001 --bootstrap 127.0.0.1:9000"

# Node C
Start-Process powershell -ArgumentList "cargo run -- --db C:\nebula_node_c --p2p 127.0.0.1:9002 --rpc 127.0.0.1:18002 --bootstrap 127.0.0.1:9000"

Write-Host ""
Write-Host "Nodes running:"
Write-Host " - Node A: p2p 127.0.0.1:9000 / rpc 127.0.0.1:18000"
Write-Host " - Node B: p2p 127.0.0.1:9001 / rpc 127.0.0.1:18001"
Write-Host " - Node C: p2p 127.0.0.1:9002 / rpc 127.0.0.1:18002"
Write-Host ""
Write-Host "Three PowerShell windows should open automatically."
Write-Host "Press Ctrl+C here to exit this launcher (nodes keep running)."
