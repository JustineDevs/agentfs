export type PublicApiDoc = {
  symbol: string;
  summary: string;
};

export function describePublicApi(doc: PublicApiDoc): string {
  return `${doc.symbol}: ${doc.summary}`;
}
