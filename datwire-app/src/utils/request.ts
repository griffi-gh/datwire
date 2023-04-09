enum Method {
  Get = "GET",
  Head = "HEAD",
  Post = "POST",
  Put = "PUT",
  Delete = "DELETE",
  Connect = "CONNECT",
  Options = "OPTIONS",
  Trace = "TRACE",
  Patch = "PATCH",
}

export async function jsonRequest(url: string, method: Method, body?: any) {
  return await fetch(url, {
    method: method,
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json"
    },
    body: body ? JSON.stringify(body) : null,
  }).then(res => res.json())
}

export async function submitFormJson(form: HTMLFormElement) {
  const formData = new FormData(form);
  const formDataObject = Object.fromEntries(formData.entries());
  const url = (<any> Method)[form.dataset.url!];
  const method = (<any> Method)[form.dataset.method!];
  return await jsonRequest(url, method, formDataObject);
}
