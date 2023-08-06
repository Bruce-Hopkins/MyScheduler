<script lang="ts">
  const date = new Date();

  function getDate(): string {
    const date = new Date();
    const day_of_the_week = getDay(date.getDay());
    const month = date.getMonth();
    const shortenMonth = getShortenedMonth(month);
    const day_of_the_month = date.getDate();

    return `${day_of_the_week} ${shortenMonth} ${day_of_the_month}`;
  }

  function getShortenedMonth(monthNumber) {
    let shortenedMonth;

    switch (monthNumber) {
      case 0:
        shortenedMonth = "Jan";
        break;
      case 1:
        shortenedMonth = "Feb";
        break;
      case 2:
        shortenedMonth = "Mar";
        break;
      case 3:
        shortenedMonth = "Apr";
        break;
      case 4:
        shortenedMonth = "May";
        break;
      case 5:
        shortenedMonth = "Jun";
        break;
      case 6:
        shortenedMonth = "Jul";
        break;
      case 7:
        shortenedMonth = "Aug";
        break;
      case 8:
        shortenedMonth = "Sep";
        break;
      case 9:
        shortenedMonth = "Oct";
        break;
      case 10:
        shortenedMonth = "Nov";
        break;
      case 11:
        shortenedMonth = "Dec";
        break;
      default:
        throw new Error(
          "Invalid month number. Please provide a number between 0 and 11."
        );
    }

    return shortenedMonth;
  }

  function getDay(number_of_day: number): string {
    switch (number_of_day) {
      case 0:
        return "Mon";
      case 1:
        return "Tue";
      case 2:
        return "Wen";
      case 3:
        return "Thu";
      case 4:
        return "Fri";
      case 5:
        return "Sat";
      case 6:
        return "Sun";
      default:
        throw new Error(
          "Invalid month number. Please provide a number between 0 and 11."
        );
    }
  }
</script>

<h3>
  {getDate()}
</h3>

<style>
</style>
