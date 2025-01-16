const baseUrl = "https://localhost:8080/"
const path = window.location.pathname;

if (path == "/login/admin") {
    document.getElementById("prefect-login-form").addEventListener("submit", async (event) => {
        event.preventDefault();
    
        const prefectEmail = document.getElementById("prefect-email").value;
        const prefectPwd = document.getElementById("prefect-pwd").value;
        const errorMessageDiv = document.getElementById("incorrect-input-div");
        const errorMessageSpan = document.getElementById("incorrect-input-span");
    
        try {
            const response = await fetch(`${baseUrl}login/admin`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
                body: new URLSearchParams({
                    "email": `${prefectEmail}`,
                    "password_hash": `${prefectPwd}`,
                })
            });
            if (response.status === 401) {
                errorMessageDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                errorMessageDiv.style.marginLeft = "2.0rem";
                errorMessageDiv.style.padding = "0.30rem";
                errorMessageDiv.style.borderRadius = "0.5rem";
                errorMessageSpan.innerHTML = "Incorrect email or password";
            } else if (response.ok) {
                errorMessageDiv.style.removeProperty("backgroundColor");
                errorMessageDiv.style.removeProperty("marginLeft");
                errorMessageDiv.style.removeProperty("padding");
                errorMessageDiv.style.removeProperty("borderRadius");
                errorMessageSpan.innerHTML = "";

                window.location.replace(`${baseUrl}admin`)
            } else {
                console.log("An unexpected error occured.")
            }
        } catch (err) {
            console.error(err)
        }
    });
}

if (path == "/login/club") {
    document.getElementById("club-login-form").addEventListener("submit", async (event) => {
        event.preventDefault();
    
        const clubEmail = document.getElementById("club-email").value;
        const clubPwd = document.getElementById("club-pwd").value;
        const errorMessageDiv = document.getElementById("club-incorrect-input-div");
        const errorMessageSpan = document.getElementById("club-incorrect-input-span");
    
        try {
            const response = await fetch(`${baseUrl}login/club`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
                body: new URLSearchParams({
                    "email": `${clubEmail}`,
                    "password_hash": `${clubPwd}`,
                })
            });
            if (response.status === 401) {
                errorMessageDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                errorMessageDiv.style.padding = "0.30rem";
                errorMessageDiv.style.borderRadius = "0.5rem";
                errorMessageSpan.innerHTML = "Incorrect email or password";
            } else if (response.ok) {
                errorMessageDiv.style.removeProperty("backgroundColor");
                errorMessageDiv.style.removeProperty("padding");
                errorMessageDiv.style.removeProperty("borderRadius");
                errorMessageSpan.innerHTML = "";

                window.location.replace(`${baseUrl}club`)
            } else {
                console.log("An unexpected error occured.")
            }
        } catch (err) {
            console.error(err)
        }
    });
}

if (path == "/register") {
    document.getElementById("registration-form").addEventListener("submit", async (event) => {
        event.preventDefault();
    
        const firstName = document.getElementById("first-name").value;
        const lastName = document.getElementById("last-name").value;
        const email = document.getElementById("email").value;
        const role = document.getElementById("roles").value;
    
        try {
            const response = await fetch(`${baseUrl}register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
                body: new URLSearchParams({
                    "email": `${clubEmail}`,
                    "password_hash": `${clubPwd}`,
                })
            });
            if (response.status === 401) {
                errorMessageDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                errorMessageDiv.style.padding = "0.30rem";
                errorMessageDiv.style.borderRadius = "0.5rem";
                errorMessageSpan.innerHTML = "Incorrect email or password";
            } else if (response.status = 303) {
                errorMessageDiv.style.removeProperty("backgroundColor");
                errorMessageDiv.style.removeProperty("padding");
                errorMessageDiv.style.removeProperty("borderRadius");
                errorMessageSpan.innerHTML = "";

                window.location.replace(`${baseUrl}create-pin`)
            } else {
                console.log("An unexpected error occured.")
            }
        } catch (err) {
            console.error(err)
        }
    });
}

