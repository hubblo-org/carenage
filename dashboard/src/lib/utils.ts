export default function format_duration(duration_in_seconds: number) {
  /* let hours = Math.floor(duration_in_seconds / 3600);
  let minutes = Math.floor((duration_in_seconds - hours * 3600) / 60);
  let seconds = duration_in_seconds - hours * 3600 - minutes * 60;
  let hours_str;
  let minutes_str;
  let seconds_str;
  if (hours < 10) {
    hours_str = "0" + hours.toString();
  }
  if (minutes < 10) {
    minutes_str = "0" + minutes.toString();
  }
  if (seconds < 10) {
    seconds_str = "0" + seconds.toString();
  }
  return hours_str + ":" + minutes_str + ":" + seconds_str; */
  const date = new Date(0);
  date.setSeconds(duration_in_seconds);
  const time_str = date.toISOString().substring(11, 19);
  return time_str;
}
