import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import { JSONRPCServer } from 'json-rpc-2.0';

const app = express();
app.use(cors());
// Parse all content types as JSON to ensure we catch Nargo's requests
app.use(bodyParser.json({ type: '*/*' }));

const server = new JSONRPCServer();
const ETH_PRICE = "2850"; 

// Log every request to see path and body
app.use((req, res, next) => {
    console.log(`[${req.method}] ${req.path}`);
    console.log('Body:', JSON.stringify(req.body).substring(0, 500));
    next();
});

server.addMethod('resolve_foreign_call', async (params) => {
    console.log('📨 Oracle called:', params); // Debug
    if (params[0].function !== 'fetchEthPrice') {
        throw new Error(`Unknown oracle: ${params[0].function}`);
    }
    // Fix: Single Field return exptects a single string in the values array, not a nested array
    return { values: [ETH_PRICE] }; 
});

// Handle all POST requests regardless of path
app.post('*', (req, res) => {
    const jsonRPCRequest = req.body;
    server.receive(jsonRPCRequest).then((jsonRPCResponse) => {
        if (jsonRPCResponse) {
            res.json(jsonRPCResponse);
        } else {
            res.sendStatus(204);
        }
    }).catch(e => {
        console.error('JSON-RPC Error:', e);
        res.status(500).send(e.message);
    });
});

app.get('/test', (req, res) => {
    res.send('🧿 Oracle Server is running. Send JSON-RPC requests to this endpoint.');
});

if (process.env.NODE_ENV !== 'production' && process.env.NODE_ENV !== 'test') {
    const port = process.env.PORT || 5555;
    app.listen(Number(port), '0.0.0.0', () => {
        console.log(`🧿 Oracle Server running!`);
        console.log(`👉 Listening on http://localhost:${port}/ (Windows)`);
        console.log(`👉 Listening on http://127.0.0.1:${port}/ (WSL/Linux compat)`);
    });
}

export default app;
