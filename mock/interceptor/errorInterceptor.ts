import { NextFunction, Request, Response } from "express";

type Error = {
  message: string;
  stack?: string;
};
export const errorInterceptor = (
  err: Error,
  req: Request,
  res: Response,
  next: NextFunction
) => {
  const error: Error = {
    message: err.message,
    stack: err.stack,
  };
  return res.status(500).json(error);
};
