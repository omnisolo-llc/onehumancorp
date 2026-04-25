BEGIN {
    skip = 0
}
/^func \(r \*VectorRepository\) GetOrganizationIDs/ {
    if (seen == 1) {
        skip = 1
    }
    seen = 1
}
{
    if (skip == 0) {
        print $0
    }
    if (skip == 1 && $0 == "}") {
        skip = 0
    }
}
