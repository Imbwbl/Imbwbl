export const DISPLAY = Symbol("std::fmt::Display");
export const DEBUG = Symbol("std::fmt::Debug");

export interface Display {
  [DISPLAY](): string;
}

export interface Debug {
  [DEBUG](): string;
}

// Helper to check if an object implements a "trait"
function formatValue(val: any, trait: typeof DISPLAY | typeof DEBUG): string {
  if (val && typeof val === "object" && trait in val) {
    return val[trait]();
  }
  return trait === DEBUG ? JSON.stringify(val, null, 2) : String(val);
}

class Formatter {
  // Regex captures: [fill/align], [sign], [#], [0], [width], [.precision], [type]
  private static specRegex =
    /^(?:(.)?([<>=^]))?([+\-])?(#)?(0)?(\d+)?(?:\.(\d+))?([a-zA-Z?])?$/;

  static format(template: string, ...args: any[]): string {
    let argIndex = 0;

    return template.replace(/{(\d+)?(?::([^}]*))?}/g, (a, index, spec) => {
      const val =
        index !== undefined ? args[parseInt(index)] : args[argIndex++];
      let output = "";

      if (!spec) return formatValue(val, DISPLAY);

      const match = spec.match(this.specRegex);
      if (!match) return formatValue(val, DISPLAY);

      const [b, fill, align, sign, alternate, zero, width, precision, type] =
        match;

      // 1. Handle Type conversion
      switch (type) {
        case "x":
          output = val.toString(16);
          break;
        case "b":
          output = val.toString(2);
          break;
        case "o":
          output = val.toString(8);
          break;
        case "X":
          output = val.toString(16).toUpperCase();
          break;
        case "?":
          output = formatValue(val, DEBUG);
          break;
        default:
          output = formatValue(val, DISPLAY);
      }

      // 2. Handle Alternate form (#)
      if (alternate) {
        if (type === "x") output = "0x" + output;
        if (type === "b") output = "0b" + output;
      }

      // 3. Handle Width and Alignment
      if (width) {
        const targetWidth = parseInt(width);
        const fillChar = fill || (zero ? "0" : " ");
        const alignment = align || (zero ? ">" : "<"); // Default Rust behavior

        output = this.applyAlignment(output, targetWidth, fillChar, alignment);
      }

      return output;
    });
  }

  private static applyAlignment(
    str: string,
    width: number,
    fill: string,
    align: string,
  ): string {
    if (str.length >= width) return str;
    const padLen = width - str.length;

    switch (align) {
      case "<":
        return str + fill.repeat(padLen);
      case ">":
        return fill.repeat(padLen) + str;
      case "^": {
        const left = Math.floor(padLen / 2);
        const right = padLen - left;
        return fill.repeat(left) + str + fill.repeat(right);
      }
      default:
        return str.padEnd(width, fill);
    }
  }
}

/**
 * Replicates format!() macro
 */
export function format(
  strings: TemplateStringsArray,
  ...values: any[]
): string {
  // Reconstruct the raw string with {} placeholders to feed the formatter
  // Or just process it directly:
  let result = "";
  for (let i = 0; i < strings.length; i++) {
    result += strings[i];
    if (i < values.length) {
      result += formatValue(values[i], DISPLAY);
    }
  }
  return result;
}

// Alternatively, a more "Rust-like" string function:
export const format_string = (str: string, ...args: any[]) =>
  Formatter.format(str, ...args);

class User implements Display, Debug {
  constructor(
    public name: string,
    public id: number,
  ) {}

  [DISPLAY]() {
    return this.name;
  }
  [DEBUG]() {
    return `User { name: "${this.name}", id: ${this.id} }`;
  }
}
