import { Router } from "express";
import { paths } from "../../definitions/generated/bitbucket";
import { ParamsDictionary } from "express-serve-static-core";
import { getGenerator } from "../../generator";
import { components } from "../../definitions/generated/bitbucket";

type BitbucketSchemas = components["schemas"];

export const bitbucketRouter = Router();

const buildStatus: (
  | "CANCELLED"
  | "FAILED"
  | "INPROGRESS"
  | "SUCCESSFUL"
  | "UNKNOWN"
)[] = ["SUCCESSFUL", "SUCCESSFUL", "SUCCESSFUL", "FAILED"];

interface LinkResponse {
  self: {
    href: string;
  }[];
}

interface PaginatedResponse<T> {
  values: T[];
}

type PullRequestUser = BitbucketSchemas["RestPullRequestParticipant"];

interface BranchInfo {
  id: string;
  displayId: string;
  latestCommit: string;
}

function generateBranchesForRepository(
  project: string,
  repository: string,
): BranchInfo[] {
  const generator = getGenerator(project + repository);
  return [
    {
      id: generator.generateId(),
      displayId: "main",
      latestCommit: generator.generateCommitHash(),
    },
    {
      id: generator.generateId(),
      displayId: `feature/${generator.generateBranchName()}`,
      latestCommit: generator.generateCommitHash(),
    },
    {
      id: generator.generateId(),
      displayId: `feature/${generator.generateBranchName()}`,
      latestCommit: generator.generateCommitHash(),
    },
    {
      id: generator.generateId(),
      displayId: `feature/${generator.generateBranchName()}`,
      latestCommit: generator.generateCommitHash(),
    },
  ];
}

function getInternal<T>(
  path: string,
  handler: (params: ParamsDictionary) => T,
) {
  const updatedPath = `/rest${path.replace(/\{(.+?)}/g, ":$1")}`;
  bitbucketRouter.get(updatedPath, (request, response) => {
    const params = request.params;
    const responseBody = handler(params);
    response.send(responseBody);
  });
}

function getPaginated<P extends keyof paths, S extends keyof BitbucketSchemas>(
  path: P,
  schema: S,
  handler: (params: ParamsDictionary) => Partial<BitbucketSchemas[S]>[],
) {
  const wrappedHandler: (
    params: ParamsDictionary,
  ) => PaginatedResponse<Partial<BitbucketSchemas[S]>> = (params) => {
    const values = handler(params);
    return {
      values,
    };
  };
  getInternal(path, wrappedHandler);
}

function get<P extends keyof paths, S extends keyof BitbucketSchemas>(
  path: P,
  schema: S,
  handler: (params: ParamsDictionary) => Partial<BitbucketSchemas[S]>,
) {
  getInternal(path, handler);
}

get(
  "/api/latest/projects/{projectKey}/repos/{repositorySlug}",
  "RestRepository",
  (params) => {
    const projectKey = params.projectKey;
    const repositorySlug = params.repositorySlug;
    const generator = getGenerator(projectKey + repositorySlug);
    const linkResponse: LinkResponse = {
      self: [
        {
          href: "https://google.ch",
        },
      ],
    };
    return {
      name: `${projectKey}/${repositorySlug}`,
      links: linkResponse as unknown as Record<string, never>,
    };
  },
);

getPaginated(
  "/api/latest/projects/{projectKey}/repos/{repositorySlug}/branches",
  "RestBranch",
  (params) => {
    const projectKey = params.projectKey;
    const repositorySlug = params.repositorySlug;
    const generator = getGenerator(projectKey + repositorySlug);
    const branches = generateBranchesForRepository(projectKey, repositorySlug);
    return branches.map((branch) => ({
      displayId: branch.displayId,
      latestCommit: branch.latestCommit,
    }));
  },
);

getPaginated(
  "/build-status/latest/commits/{commitId}",
  "RestBuildStatus",
  (params) => {
    const commitId = params.commitId;
    const generator = getGenerator(commitId);

    return [
      {
        state: generator.pickRandomArrayElement(buildStatus),
        url: "https://bitbucket.com",
      },
    ];
  },
);

getPaginated(
  "/api/latest/projects/{projectKey}/repos/{repositorySlug}/pull-requests",
  "RestPullRequest",
  (params) => {
    const projectKey = params.projectKey;
    const repositorySlug = params.repositorySlug;

    const generator = getGenerator(projectKey + repositorySlug);

    const author: PullRequestUser = {
      approved: false,
      user: {
        displayName: generator.generateName(),
        slug: generator.generateId(),
      },
    };
    const linkResponse: LinkResponse = {
      self: [
        {
          href: "https://google.ch",
        },
      ],
    };

    const branches = generateBranchesForRepository(projectKey, repositorySlug);
    const sourceBranch = branches[1];
    const targetBranch = branches[0];

    return [
      {
        id: generator.randomPositiveInt(),
        fromRef: sourceBranch,
        toRef: targetBranch,
        author: author,
        reviewers: [
          {
            approved: generator.pickRandomArrayElement([true, false, false]),
            user: {
              displayName: generator.generateName(),
              slug: generator.generateId(),
            },
          },
        ],
        updatedDate: generator.randomPositiveInt(),
        links: linkResponse as unknown as Record<string, never>,
        properties: {
          comment_count: generator.randomIntInRange(0, 15),
        },
      },
    ];
  },
);

get("/api/latest/users/{userSlug}", "RestApplicationUser", () => {
  return {
    avatarUrl:
      "https://www.gravatar.com/avatar/205e460b479e2e5b48aec07710c08d50.jpg?s=32",
  };
});
