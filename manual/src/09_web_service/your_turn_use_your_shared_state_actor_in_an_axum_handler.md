# Your Turn: Use your shared state actor in an Axum handler!

> My version is in `code/axum_with_my_actor`.

What you want to try and accomplish:
* Import the shared state actor you created earlier.
* Create an instance of the actor and add it as a layer to your Axum router.
* Modify the `hello_json` handler to recieve the layer.
* Use the actor to call the increment function.
* Get the new total.
* Return the new total in the JSON response.

This pulls together a lot of what we've done so far. Good luck! I'll be here to help.

---

Let's take a look at my solution in `code/axum_with_my_actor`.