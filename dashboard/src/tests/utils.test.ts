import { getMetricUnit } from "$lib/utils";
import { it, expect } from "vitest";

it("strips metricName to last digits to get the unit of the metric", () => {
  const metricName = "average_power_measure_w";
  const metricUnit = getMetricUnit(metricName);
  expect(metricUnit).toEqual("w");
});
