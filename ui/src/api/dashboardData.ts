import type { DashboardData } from "../types";

export async function getDashboardData(): Promise<DashboardData> {
  const res = await fetch(`/api/dashboard-data`);
  const text = await res.text();

  if (res.ok) {
    return JSON.parse(text);
  } else {
    throw new Error(text);
  }
}
