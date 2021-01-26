import { Email, Project } from "./types";
import { API } from "./api.json";
import axios from "axios";

export async function getProjects(): Promise<Array<Project>> {
  return await axios.get<Array<Project>>(`${API}/get/projects`).then((res) => {
    return res.data;
  });
}

export async function sendEmail(email: Email): Promise<string> {
  return await axios.post<string>(`${API}/send_email`, email)
    .then(res => {return res.data})
    .then(e => {return e})
}
