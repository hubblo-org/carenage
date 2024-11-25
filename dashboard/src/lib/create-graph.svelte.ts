import type { MetricValues } from "../types/carenage";

export async function createGraph(metricValues: MetricValues, metric: string) {
  const Dygraph = (await import("dygraphs")).default;
  const values = metricValues.map((value) => {
    const date = new Date(value[0]);
    return [date, value[1]];
  });
  console.log(metric);
  const dygraph = new Dygraph("graph", values, {
    labels: ["timestamp", "metric_value"],
    title: "Metric values for selected process and metric",
    xlabel: "Time",
    ylabel: `${metric}`,
    height: 700
  });
  return dygraph;
}
