import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import { JSONRPCServer } from 'json-rpc-2.0';

const app = express();
app.use(cors());
app.use(bodyParser.json());

const server = new JSONRPCServer();
const ETH_PRICE = "2850.50";  // Mock - replace w/ Pyth

server.addMethod('resolve_foreign_call', async (params) => {
  if (params[0].function !== 'fetchEthPrice') 
    throw Error('Unknown oracle');
  return { values: [[ETH_PRICE]] };
});

app.post('/oracle', (req, res) => {
  server.receive(req.body).then((jsonRPCResponse) => {
    if (jsonRPCResponse) res.json(jsonRPCResponse);
    else res.status(204).end();
  });
});

const port = process.env.PORT || 5555;
app.listen(port, () => console.log(`Runnning the Oracle at PORT:${port}`));
