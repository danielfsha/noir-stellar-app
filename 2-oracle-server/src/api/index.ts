import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import { JSONRPCServer } from 'json-rpc-2.0';

const app = express();
app.use(cors());
app.use(bodyParser.json());

const server = new JSONRPCServer();
const ETH_PRICE = "2850"; 

server.addMethod('resolve_foreign_call', async (params) => {
    console.log('📨 Oracle called:', params); // Debug
    if (params[0].function !== 'fetchEthPrice') {
        throw new Error(`Unknown oracle: ${params[0].function}`);
    }
    // Fix: Single Field return exptects a single string in the values array, not a nested array
    return { values: [ETH_PRICE] }; 
});

app.post('/', (req, res) => {  // Nargo expects ROOT endpoint
    console.log('Request:', req.body); // Debug
    server.receive(req.body).then((jsonRPCResponse) => {
        if (jsonRPCResponse) res.json(jsonRPCResponse);
        else res.status(204).end();
    });
});

app.get('/test', (req, res) => {
    res.send('🧿 Oracle Server is running. Send JSON-RPC requests to this endpoint.');
});

if (process.env.NODE_ENV !== 'production') {
    const port = process.env.PORT || 5555;
    app.listen(Number(port), '0.0.0.0', () => {
        console.log(`🧿 Oracle Server running!`);
        console.log(`👉 Listening on http://localhost:${port}/ (Windows)`);
        console.log(`👉 Listening on http://127.0.0.1:${port}/ (WSL/Linux compat)`);
    });
}

export default app;
