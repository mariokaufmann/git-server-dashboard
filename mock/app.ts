import express, { Express, Request, Response } from "express";
import * as swaggerUiExpress from "swagger-ui-express";
import { errorInterceptor } from "./interceptor/errorInterceptor";
import { bitbucketRouter } from "./router/bitbucket/bitbucketRouter";
import {bitbucketWebhookTrigger} from "./router/bitbucket/bitbucketWebhookTrigger";

const app: Express = express();
const port = 3001;

app.get("/home", (req: Request, res: Response) => res.send("mock server"));

app.get("/swagger/definition", (req: Request, res: Response) => {
  res.sendFile(`${process.cwd()}/definitions/bitbucket-8.10.swagger.v3.json`);
});

app.use(
  "/swagger",
  swaggerUiExpress.serve,
  swaggerUiExpress.setup(undefined, {
    swaggerOptions: {
      displayRequestDuration: true,
      url: "/swagger/definition",
    },
  })
);

app.use(express.json());
app.use(bitbucketRouter);
app.use(bitbucketWebhookTrigger);
app.use(errorInterceptor);

app.listen(port, function () {
  console.log(`Mock server listening on port ${port}!`);
});
