import { getMetricUnit, isUUID } from "$lib/utils";
import { describe, it, expect } from "vitest";

it("strips metricName to last digits to get the unit of the metric", () => {
  const metricName = "average_power_measure_w";
  const metricUnit = getMetricUnit(metricName);
  expect(metricUnit).toEqual("w");
});

describe("isUUID function test suite", () => {
  it("asserts that a string is parsable as an UUID", () => {
    const validUuid = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
    const isUuid = isUUID(validUuid);
    expect(isUuid).toBeTruthy();
  });
  it("checks that a string is not parsable as an UUID", () => {
    const strings = ["invalid_id", "0123456789", "3a1f9a71-fdd2-4e89-9769"];
    strings.forEach((string) => {
      const isUuid = isUUID(string);
      expect(isUuid).toBeFalsy();
    });
  });
});
