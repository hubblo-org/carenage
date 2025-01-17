import type { UUID } from "crypto";

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

export function formatDate(date: string) {
  const str_to_date = new Date(date);
  const formattedDate = str_to_date.toLocaleDateString("en-US");
  return formattedDate;
}

export function formatTime(date: string) {
  const str_to_date = new Date(date);
  return str_to_date;
}

export function isUUID(uuid: string): uuid is UUID {
  return /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(uuid);
}
