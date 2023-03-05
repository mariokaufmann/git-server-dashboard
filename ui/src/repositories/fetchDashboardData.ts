import type { DashboardData } from '../types';
import { estimateLineCount } from './utils';

export async function getDashboardData(): Promise<DashboardData> {
  const res = await fetch(`/api/dashboard-data`);
  const text = await res.text();

  if (res.ok) {
    const data: DashboardData = JSON.parse(text);
    return {
      ...data,
      repositories: data.repositories.sort(
        (rep1, rep2) => estimateLineCount(rep2) - estimateLineCount(rep1)
      ),
    };
  } else {
    throw new Error(text);
  }
}
