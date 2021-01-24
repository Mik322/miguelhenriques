import { Email, Project } from "./types"


//const API = "http://192.168.56.1:8080"

const projects = [
    {id: 1, name: "Site", imageName: "Site.png", description: "This website"}, 
    {id: 2, name: "QuarentineLife", description: "A forum created for a school project"}];

export function getProjects(): Array<Project> {
    return projects
}

export function getProjectById(id: number): Project | null {
    return projects[id-1]
}

export function sendEmail(email: Email) {
    
}