import { Email, Project } from "./types"


//const API = "http://192.168.56.1:8080"

export function getProjects(): Array<Project> {
    return [{id: 1, name: "Site", imageName: "Site.png", description: "This website"}, 
    {id: 2, name: "QuarentineLife", description: "A forum created for a school project"}]
}

export function getProjectById(id: Number): any {
    return null
}

export function sendEmail(email: Email) {
    
}