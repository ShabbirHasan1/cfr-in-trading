import typing

import prettytable as pt


class Item(typing.NamedTuple):
    iteration: int
    mean_length: float
    mean_prediction: float


def output_to_pretty_table(text: str) -> str:
    """Output the text to a pretty table."""
    items = []
    lines = text.splitlines()
    index = 0
    while index < len(lines):
        line = lines[index]
        if line.startswith("Iteration: "):
            iteration = int(line.split(": ")[1])
            mean_length = round(float(lines[index + 2].split(": ")[1]), 2)
            mean_prediction = round(float(lines[index + 3].split(": ")[1]), 4)
            item = Item(iteration, mean_length, mean_prediction)
            print(item)
            items.append(item)
            index += 4
        else:
            index += 1

    table = pt.PrettyTable(["Iteration", "Mean Play Length", "Mean Utility Prediction"])
    for item in items:
        table.add_row([item.iteration, item.mean_length, item.mean_prediction])
    return str(table)


def get_multiline_input() -> str:
    """Get a multiline input from the user."""
    lines = []
    while True:
        try:
            line = input()
        except EOFError:
            break
        lines.append(line)
    return "\n".join(lines)


if __name__ == "__main__":
    print("Paste the output here and then press ctrl+d")
    text = get_multiline_input()
    table = output_to_pretty_table(text)
    print(table)
