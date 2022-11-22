export interface DashboardData {
  last_updated_date?: string;
  repositories: RepositoryBranchData[];
  currently_refreshing: boolean;
}

export interface RepositoryBranchData {
  repository_name: string;
  repository_url: string;
  pull_request_target_branches: PullRequestTargetBranch[];
  standalone_branches: StandaloneBranch[];
}

export interface PullRequestTargetBranch {
  branch_name: string;
  pipeline_status: PipelineStatus;
  pipeline_url?: string;
  pull_requests: PullRequest[];
}

export interface PullRequest {
  branch_name: string;
  user_name: string;
  user_profile_image: string;
  comment_count: number;
  last_activity_date: string;
  approved: boolean;
  pipeline_status: PipelineStatus;
  pipeline_url?: string;
  link_url: string;
}

export interface StandaloneBranch {
  branch_name: string;
  pipeline_status: PipelineStatus;
  pipeline_url?: string;
}

export type PipelineStatus =
  | "Running"
  | "Successful"
  | "Failed"
  | "None"
  | "Canceled"
  | "Queued";
