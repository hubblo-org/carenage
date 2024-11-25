export function formatDuration(duration_in_seconds: number) {
  const date = new Date(0);
  date.setSeconds(duration_in_seconds);
  const time_str = date.toISOString().substring(11, 19);
  return time_str;
}

export function getMetricUnit(metric: string) {
  const underscore_offset = metric.lastIndexOf("_");
  const metric_unit = metric.substring(underscore_offset + 1, metric.length);
  return metric_unit;
}

export function average(array: number[]) {
  const averageValue = array.reduce((a: number, b: number) => a + b) / array.length;
  return averageValue;
}
