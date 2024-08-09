export function adjustTextAreaHeight(textArea: HTMLTextAreaElement) {
  if (textArea === null) return;

  textArea.style.height = "auto";
  textArea.style.height = `${textArea.scrollHeight}px`;
}
