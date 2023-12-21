import { RepositoryBranchData } from '../types';

export function estimateLineCount(repository: RepositoryBranchData) {
  return (
    repository.standalone_branches.length +
    repository.pull_request_target_branches.length +
    repository.pull_request_target_branches.reduce(
      (previous, current) => previous + 2 * current.pull_requests.length,
      0,
    )
  );
}
