export type ButtonVariant = "primary" | "secondary";

export function buttonClass(variant: ButtonVariant): string {
  return variant === "primary" ? "button button-primary" : "button button-secondary";
}
