const baseUrl = "https://localhost:8080/"
const path = window.location.pathname;

if (path == "/register") {
    document.getElementById("submit-auth-btn").addEventListener("click", async (event) => {
        event.preventDefault();

        const invalidIdDiv = document.getElementById("invalid-id-div");
        const invalidIdSpan = document.getElementById("invalid-id-span");
        const idChars = document.querySelectorAll('#auth-row .auth-input');        
        const combinedIdChars = Array.from(idChars).map(idChars => idChars.value).join('');

        try {
            const response = await fetch(`${baseUrl}register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
                body: new URLSearchParams({
                    "value": `${combinedIdChars}`,
                })
            });
            if (response.status === 401) {
                invalidIdDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                invalidIdDiv.style.marginLeft = "2.0rem";
                invalidIdDiv.style.padding = "0.30rem";
                invalidIdDiv.style.borderRadius = "0.5rem";
                invalidIdSpan.innerHTML = "Invalid registration ID";
            } else if (response.ok) {
                invalidIdDiv.style.removeProperty("backgroundColor");
                invalidIdDiv.style.removeProperty("marginLeft");
                invalidIdDiv.style.removeProperty("padding");
                invalidIdDiv.style.removeProperty("borderRadius");
                invalidIdSpan.innerHTML = "";

                window.location.replace(`${baseUrl}create-pin`)
            } else {
                console.log("An unexpected error occured.")
            }
        } catch (err) {
            console.error(err)
        }
    })
}

if (path == "/create-pin") {
    document.getElementById("password-form").addEventListener("submit", async (event) => {
        event.preventDefault();

        const newPassword = document.getElementById("new-password").value;
        const confirmPassword = document.getElementById("confirm-password").value;
        const passwordsDontMatchDiv = document.getElementById("passwords-dont-match-div");
        const passwordsDontMatchSpan = document.getElementById("passwords-dont-match-span");
        
        try {
            const response = await fetch(`${baseUrl}create-pin`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
                body: new URLSearchParams({
                    "new_password": `${newPassword}`,
                    "confirm_password": `${confirmPassword}`,
                })
            });
            if (response.status === 401) {
                passwordsDontMatchDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                passwordsDontMatchDiv.style.marginLeft = "2.0rem";
                passwordsDontMatchDiv.style.padding = "0.30rem";
                passwordsDontMatchDiv.style.borderRadius = "0.5rem";
                passwordsDontMatchSpan.innerHTML = "The passwords don't match";
            } else if (response.ok) {
                passwordsDontMatchDiv.style.removeProperty("backgroundColor");
                passwordsDontMatchDiv.style.removeProperty("marginLeft");
                passwordsDontMatchDiv.style.removeProperty("padding");
                passwordsDontMatchDiv.style.removeProperty("borderRadius");
                passwordsDontMatchSpan.innerHTML = "";

                console.log(`Location: ${response.url}`);
                if (response.url === `${baseUrl}login/club`) {
                    window.location.replace(`${baseUrl}login/club`)
                } else {
                    window.location.replace(`${baseUrl}login/admin`)
                }
            } else {
                console.log("An unexpected error occured.")
            }
        } catch (err) {
            console.error(err)
        }
    })
}

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