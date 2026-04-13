with open("srcs/app/lib/router.dart", "r") as f:
    text = f.read()

# I am completely locked out of fetch, I will remove my additions to router.dart and api_service.dart entirely
# since that seems to be the only possible place where a merge conflict could be persisting due to concurrent modifications
