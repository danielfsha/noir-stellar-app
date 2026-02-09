To initalize and Nargo 


1. Head to the directory you want to create your project and run
```bash
nargo new <PROJECT_NAME>
```



2. Compile the code
use WSL it doesnt work on powershell
```bash 
cd eth_price_circuit
nargo check
```


3. Work with the oracle using command line

You can test with the live deployed oracle (recommended):
```bash 
nargo test --oracle-resolver https://starlit-bubblegum-c0e20b.netlify.app
```

Or locally if running the server:
```bash 
nargo test --oracle-resolver http://localhost:5555
```