const baseUrl = "http://localhost:8080/"
const authUrl = "http://localhost:8000/"
const path = window.location.pathname;

if (path == "/register") {
    document.getElementById("submit-auth-btn").addEventListener("click", async (event) => {
        event.preventDefault();

        const invalidIdDiv = document.getElementById("invalid-id-div");
        const invalidIdSpan = document.getElementById("invalid-id-span");
        const idChars = document.querySelectorAll('#auth-row .auth-input');        
        const combinedIdChars = Array.from(idChars).map(idChars => idChars.value).join('');

        try {
            const response = await fetch(`${authUrl}register`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "value": `${combinedIdChars}`,
                })
            })
            .then(async res  => {
                const contentType = res.headers.get("content-type")

                if (contentType && contentType.includes("application/json")) {
                    const data = await res.json();
                    if (data.redirect) {
                        window.location.href = data.redirect;
                    }

                    if (res.status === 401) {
                        invalidIdDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                        invalidIdDiv.style.marginLeft = "2.0rem";
                        invalidIdDiv.style.padding = "0.30rem";
                        invalidIdDiv.style.borderRadius = "0.5rem";
                        invalidIdSpan.innerHTML = "Invalid registration ID";
                    } else if (res.ok) {
                        invalidIdDiv.style.removeProperty("backgroundColor");
                        invalidIdDiv.style.removeProperty("marginLeft");
                        invalidIdDiv.style.removeProperty("padding");
                        invalidIdDiv.style.removeProperty("borderRadius");
                        invalidIdSpan.innerHTML = "";

                        window.location.replace(`${baseUrl}create-pin`)
                    } else {
                        console.log("An unexpected error occured.")
                    }
                }
            })
            
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
            const response = await fetch(`${authUrl}create-pin`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "new_password": `${newPassword}`,
                    "confirm_password": `${confirmPassword}`,
                })
            })
            .then(async res => {
                const contentType = res.headers.get("content-type")
                
                if (contentType && contentType.includes("application/json")) {
                    const data = await res.json();
                    if (data.redirect) {
                        console.log(data.redirect)
                        window.location.href = data.redirect;
                    }

                    if (res.status === 401) {
                        passwordsDontMatchDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                        passwordsDontMatchDiv.style.marginLeft = "2.0rem";
                        passwordsDontMatchDiv.style.padding = "0.30rem";
                        passwordsDontMatchDiv.style.borderRadius = "0.5rem";
                        passwordsDontMatchSpan.innerHTML = "The passwords don't match";
                    } else if (res.ok) {
                        passwordsDontMatchDiv.style.removeProperty("backgroundColor");
                        passwordsDontMatchDiv.style.removeProperty("marginLeft");
                        passwordsDontMatchDiv.style.removeProperty("padding");
                        passwordsDontMatchDiv.style.removeProperty("borderRadius");
                        passwordsDontMatchSpan.innerHTML = "";

                        if (res.url === `${authUrl}login/club`) {
                            window.location.replace(`${baseUrl}login/club`)
                        } else {
                            window.location.replace(`${baseUrl}login/prefect`)
                        }
                    } else {
                        console.log("An unexpected error occured.")
                    }
                        }
                    })
                    
        } catch (err) {
            console.error(err)
        }
    })
}

if (path == "/login/prefect") {
    document.getElementById("prefect-login-form").addEventListener("submit", async (event) => {
        event.preventDefault();
    
        const prefectEmail = document.getElementById("prefect-email").value;
        const prefectPwd = document.getElementById("prefect-pwd").value;
        const errorMessageDiv = document.getElementById("incorrect-input-div");
        const errorMessageSpan = document.getElementById("incorrect-input-span");
    
        try {
            const response = await fetch(`${authUrl}login/prefect`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "email": `${prefectEmail}`,
                    "password_hash": `${prefectPwd}`,
                }),
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },
            })
            .then(async res => {
                const contentType = res.headers.get("content-type")

                if (contentType && contentType.includes("application/json")) {
                    const data = await res.json();
                    if (data.redirect) {
                        window.location.href = data.redirect;
                    }
                } else {
                    if (res.status === 401) {
                        errorMessageDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                        errorMessageDiv.style.marginLeft = "2.0rem";
                        errorMessageDiv.style.padding = "0.30rem";
                        errorMessageDiv.style.borderRadius = "0.5rem";
                        errorMessageSpan.innerHTML = "Incorrect email or password";
                    } else if (res.ok) {
                        errorMessageDiv.style.removeProperty("backgroundColor");
                        errorMessageDiv.style.removeProperty("marginLeft");
                        errorMessageDiv.style.removeProperty("padding");
                        errorMessageDiv.style.removeProperty("borderRadius");
                        errorMessageSpan.innerHTML = "";

                        window.location.replace(`${baseUrl}prefect`)
                    } else {
                        console.log("An unexpected error occured.")
                    }
                }
            }) 
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
            const response = await fetch(`${authUrl}login/club`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "email": `${clubEmail}`,
                    "password_hash": `${clubPwd}`,
                }),
                headers: {
                    "Content-Type": "application/x-www-form-urlencoded"
                },  
            })
            .then(async res => {
                const contentType = res.headers.get("content-type")

                if (contentType && contentType.includes("application/json")) {
                    const data = await res.json();
                    if (data.redirect) {
                        window.location.href = data.redirect;
                    }
                } else {
                    if (res.status === 401) {
                        errorMessageDiv.style.backgroundColor = "rgba(90,0,0,0.3)";
                        errorMessageDiv.style.padding = "0.30rem";
                        errorMessageDiv.style.borderRadius = "0.5rem";
                        errorMessageSpan.innerHTML = "Incorrect email or password";
                    } else if (res.ok) {
                        errorMessageDiv.style.removeProperty("backgroundColor");
                        errorMessageDiv.style.removeProperty("padding");
                        errorMessageDiv.style.removeProperty("borderRadius");
                        errorMessageSpan.innerHTML = "";

                        window.location.replace(`${baseUrl}club`)
                    } else {
                        console.log("An unexpected error occured.")
                    }
                }
            }) 
        } catch (err) {
            console.error(err)
        }
    });
}

if (path == "/club") {
    document.getElementById("announcement-form").addEventListener("submit", async (event) => {
        event.preventDefault();

        const announcementInfo = document.getElementById('announcement').value;        
        const announcementDate = document.getElementById('announcement-form-date').value;

        try {
            await fetch(`${authUrl}club`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "info": `${announcementInfo}`,
                    "date": `${announcementDate}`,
                })
            });
        } catch (err) {
            console.error(err)
        }
        location.reload();
    })
}

if (path == "/prefect") {
    document.getElementById("announcement-form").addEventListener("submit", async (event) => {
        event.preventDefault();

        const announcingClub = document.getElementById('announcing-club').value;
        const announcementInfo = document.getElementById('announcement').value;        
        const announcementDate = document.getElementById('announcement-form-date').value;

        try {
            await fetch(`${authUrl}prefect`, {
                method: "POST",
                credentials: "include",
                body: new URLSearchParams({
                    "name": `${announcingClub}`,
                    "info": `${announcementInfo}`,
                    "date": `${announcementDate}`,
                })
            });
        } catch (err) {
            console.error(err)
        }
        location.reload();
    })
}

if (path == "/logout") {
    document.getElementById("sign-out-btn").addEventListener("click", async (event) => {
        event.preventDefault();

        try {
            await fetch(`${authUrl}logout`, {
                method: "POST",
                credentials: "include"
            })
            .then(async res => {
                const contentType = res.headers.get("content-type")

                if (contentType && contentType.includes("application/json")) {
                    const data = await res.json();
                    if (data.redirect) {
                        window.location.href = data.redirect;
                    }
                }    
            })
        } catch (err) {
            console.error(err)
        }
        window.location.replace(`${baseUrl}`)
    })
}