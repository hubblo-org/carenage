import type { MetricValues } from "../types/carenage";

export async function createGraph(metricValues: MetricValues) {
  const Dygraph = (await import("dygraphs")).default;
  const values = metricValues.map((value) => {
    const date = new Date(value[0]);
    return [date, value[1]];
  });
  const dygraph = new Dygraph("graph", values, {
    labels: ["timestamp", "metric_value"]
  });
  return dygraph;
}
