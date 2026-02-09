import serverless from 'serverless-http';
import app from '../../src/api/index.js'; // Note .js extension for ESM resolution often required, or check tsconfig

export const handler = serverless(app);
