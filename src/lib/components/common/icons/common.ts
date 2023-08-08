export function getIconSizeStyles(size?: number) {
  if (size) {
    return `height: ${size}px; width: ${size}px;`;
  } else "";
}
