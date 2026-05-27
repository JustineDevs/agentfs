type PublicHealthResponse = {
  ok: true;
  service: "agentfs-api";
};

export function health(): PublicHealthResponse {
  return {
    ok: true,
    service: "agentfs-api"
  };
}
