export function routeIntent(message: string): { department_assigned: string, agent: string, description: string } {
    const lowerMessage = message.toLowerCase();

    if (lowerMessage.includes('quote') || lowerMessage.includes('lead')) {
        return {
            department_assigned: 'sales',
            agent: 'The Salesperson',
            description: `Handle ${lowerMessage.includes('quote') ? 'quote' : 'lead'} for sales`
        };
    }

    if (lowerMessage.includes('email') || lowerMessage.includes('campaign')) {
        return {
            department_assigned: 'marketing',
            agent: 'The Promoter',
            description: `Manage marketing ${lowerMessage.includes('email') ? 'email' : 'campaign'}`
        };
    }

    if (lowerMessage.includes('refund')) {
        return {
            department_assigned: 'finance',
            agent: 'The Accountant',
            description: 'Process refund request'
        };
    }

    if (lowerMessage.includes('contract')) {
        return {
            department_assigned: 'legal',
            agent: 'The Protector',
            description: 'Review legal contract'
        };
    }

    if (lowerMessage.includes('insight') || lowerMessage.includes('performance')) {
        return {
            department_assigned: 'business_advisory',
            agent: 'The Advisor',
            description: 'Provide business performance insight'
        };
    }

    if (lowerMessage.includes('dm') || lowerMessage.includes('customer')) {
        return {
            department_assigned: 'customer_success',
            agent: 'The Ambassador',
            description: 'Respond to customer dm'
        };
    }

    if (lowerMessage.includes('urgent') || lowerMessage.includes('emergency')) {
        return {
            department_assigned: 'triage',
            agent: 'The Coordinator',
            description: 'Urgent task prioritization'
        };
    }

    return {
        department_assigned: 'operations',
        agent: 'The Manager',
        description: 'Update operations schedule'
    };
}
