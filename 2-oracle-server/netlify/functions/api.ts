import serverless from 'serverless-http';
import app from '../../src/api/index'; // Let module resolution handle extensions

export const handler = serverless(app);
