document.querySelectorAll('.copy-link').forEach((copyLinkParent) => {
  const inputField = copyLinkParent.querySelector('.copy-link-input');
  const copyButton = copyLinkParent.querySelector('.copy-link-button');
  inputField.href = inputField.innerHTML;
  const text = inputField.href;

  copyButton.addEventListener('click', () => {
    navigator.clipboard.writeText(text);
    inputField.innerHTML = '<b>Copied!</b>';
    setTimeout(() => (inputField.innerHTML = text), 2000);
  });
});
