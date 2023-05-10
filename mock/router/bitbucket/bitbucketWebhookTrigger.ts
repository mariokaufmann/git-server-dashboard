import {Router} from "express";
import {getGenerator} from "../../generator";

const GIT_SERVER_DASHBOARD_URL = "http://127.0.0.1:8080/webhook/bitbucket";
export const bitbucketWebhookTrigger = Router();

type EventType = "OPENED" | "MERGED" | "UPDATED" | "COMMENTED" | "APPROVED";
const eventKeyMappings: { [key in EventType]: string } = {
    OPENED: "pr:opened",
    MERGED: "pr:merged",
    UPDATED: "pr:from_ref_updated",
    COMMENTED: "pr:commented:added",
    APPROVED: "pr:reviewer:approved"
};

interface WebhookTriggerRequest {
    project: {
        id: number;
    }
    repository: {
        id: number;
        name: string;
    }
    event: {
        date: string;
        type: EventType;
    }
    detail: string;
}

interface BitbucketWebhookPayload {
    eventKey: string,
    actor: {
        displayName: string;
    },
    pullRequest: PullRequestPayload,
}

interface PullRequestPayload {
    id: number,
    title: string
    fromRef: GitRefPayload,
    toRef: GitRefPayload,
    links: {
        self: { href: string }[],
    }
}

interface GitRefPayload {
    repository: {
        id: number,
        name: string,
        project: {
            id: number
        }
    }
}

bitbucketWebhookTrigger.post(
    "/webhook-trigger/bitbucket",
    (request, response) => {
        const requestBody = request.body as WebhookTriggerRequest;
        const generator = getGenerator('');

        const webhookPayload: BitbucketWebhookPayload = {
            eventKey: eventKeyMappings[requestBody.event.type],
            actor: {
                displayName: generator.generateName(),
            },
            pullRequest: {
                id: generator.randomPositiveInt(),
                title: generator.generateCommitMessage(),
                fromRef: {
                    repository: {
                        id: requestBody.repository.id,
                        name: requestBody.repository.name,
                        project: {
                            id: requestBody.project.id,
                        }
                    }
                },
                toRef: {
                    repository: {
                        id: requestBody.repository.id,
                        name: requestBody.repository.name,
                        project: {
                            id: requestBody.project.id
                        }
                    }
                },
                links: {
                    self: [
                        {href: "https://bitbucket.com"}
                    ]
                }
            }
        };

        fetch(GIT_SERVER_DASHBOARD_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(webhookPayload),
        }).then()

        response.send();
    }
);
