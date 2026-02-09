import serverless from 'serverless-http';
import app from '../../src/api/index.js'; // Require .js extension for ESM/NodeNext

export const handler = serverless(app);
