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
```bash 
nargo test --oracle-resolver http://localhost:5555
```