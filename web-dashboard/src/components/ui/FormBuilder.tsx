import React from 'react'
import { useForm, FieldPath, FieldValues, Control } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { cn } from '@/lib/utils'

export interface FormFieldConfig<T extends FieldValues> {
  name: FieldPath<T>
  type: 'text' | 'email' | 'password' | 'number' | 'textarea' | 'select' | 'checkbox' | 'radio' | 'file' | 'date'
  label: string
  placeholder?: string
  description?: string
  required?: boolean
  disabled?: boolean
  options?: Array<{ value: string; label: string }>
  validation?: z.ZodType<any>
  className?: string
  rows?: number
  accept?: string
  multiple?: boolean
}

export interface FormBuilderProps<T extends FieldValues> {
  fields: FormFieldConfig<T>[]
  schema: z.ZodType<T>
  onSubmit: (data: T) => void | Promise<void>
  defaultValues?: Partial<T>
  submitText?: string
  resetText?: string
  loading?: boolean
  className?: string
  gridCols?: 1 | 2 | 3 | 4
}

export function FormBuilder<T extends FieldValues>({
  fields,
  schema,
  onSubmit,
  defaultValues,
  submitText = 'Submit',
  resetText = 'Reset',
  loading = false,
  className,
  gridCols = 1,
}: FormBuilderProps<T>) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
    reset,
    control,
  } = useForm<T>({
    resolver: zodResolver(schema),
    defaultValues,
  })

  const gridClasses = {
    1: 'grid-cols-1',
    2: 'grid-cols-1 md:grid-cols-2',
    3: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3',
    4: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4',
  }

  const renderField = (field: FormFieldConfig<T>) => {
    const error = errors[field.name]
    const fieldProps = {
      ...register(field.name),
      disabled: field.disabled || loading,
      className: cn(
        'input',
        error && 'input-error',
        field.className
      ),
    }

    const fieldId = `field-${String(field.name)}`

    return (
      <div key={String(field.name)} className="space-y-2">
        <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 dark:text-gray-300">
          {field.label}
          {field.required && <span className="text-red-500 ml-1">*</span>}
        </label>

        {field.description && (
          <p className="text-sm text-gray-500 dark:text-gray-400">
            {field.description}
          </p>
        )}

        {field.type === 'textarea' ? (
          <textarea
            {...fieldProps}
            id={fieldId}
            placeholder={field.placeholder}
            rows={field.rows || 3}
          />
        ) : field.type === 'select' ? (
          <select {...fieldProps} id={fieldId}>
            <option value="">{field.placeholder || 'Select an option'}</option>
            {field.options?.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        ) : field.type === 'checkbox' ? (
          <div className="flex items-center space-x-2">
            <input
              {...fieldProps}
              type="checkbox"
              id={fieldId}
              className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
            />
            <label htmlFor={fieldId} className="text-sm text-gray-700 dark:text-gray-300">
              {field.placeholder || field.label}
            </label>
          </div>
        ) : field.type === 'radio' ? (
          <div className="space-y-2">
            {field.options?.map((option) => (
              <div key={option.value} className="flex items-center space-x-2">
                <input
                  {...fieldProps}
                  type="radio"
                  value={option.value}
                  id={`${fieldId}-${option.value}`}
                  className="border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <label htmlFor={`${fieldId}-${option.value}`} className="text-sm text-gray-700 dark:text-gray-300">
                  {option.label}
                </label>
              </div>
            ))}
          </div>
        ) : field.type === 'file' ? (
          <input
            {...fieldProps}
            type="file"
            id={fieldId}
            accept={field.accept}
            multiple={field.multiple}
            className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-primary-50 file:text-primary-700 hover:file:bg-primary-100 dark:file:bg-primary-900/20 dark:file:text-primary-400"
          />
        ) : (
          <input
            {...fieldProps}
            type={field.type}
            id={fieldId}
            placeholder={field.placeholder}
          />
        )}

        {error && (
          <p className="text-sm text-red-600 dark:text-red-400">
            {error.message}
          </p>
        )}
      </div>
    )
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className={cn('space-y-6', className)}>
      <div className={cn('grid gap-6', gridClasses[gridCols])}>
        {fields.map(renderField)}
      </div>

      <div className="flex items-center justify-end space-x-3 pt-6 border-t border-gray-200 dark:border-gray-700">
        <button
          type="button"
          onClick={() => reset()}
          disabled={loading || isSubmitting}
          className="btn btn-secondary"
        >
          {resetText}
        </button>
        <button
          type="submit"
          disabled={loading || isSubmitting}
          className="btn btn-primary"
        >
          {isSubmitting ? (
            <>
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2" />
              Processing...
            </>
          ) : (
            submitText
          )}
        </button>
      </div>
    </form>
  )
}

// Example usage with predefined common form schemas
export const userFormSchema = z.object({
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
  confirmPassword: z.string(),
  role: z.enum(['admin', 'user', 'viewer']),
  bio: z.string().optional(),
  agreeToTerms: z.boolean().refine(val => val === true, 'You must agree to the terms'),
}).refine(data => data.password === data.confirmPassword, {
  message: 'Passwords do not match',
  path: ['confirmPassword'],
})

export const projectFormSchema = z.object({
  name: z.string().min(1, 'Project name is required'),
  description: z.string().min(10, 'Description must be at least 10 characters'),
  language: z.string().min(1, 'Language is required'),
  framework: z.string().min(1, 'Framework is required'),
  repository: z.string().url('Invalid repository URL').optional().or(z.literal('')),
  private: z.boolean(),
  tags: z.string().optional(),
})

export const contactFormSchema = z.object({
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  subject: z.string().min(5, 'Subject must be at least 5 characters'),
  message: z.string().min(20, 'Message must be at least 20 characters'),
  priority: z.enum(['low', 'medium', 'high']),
  newsletter: z.boolean().optional(),
})

// Quick form builder components for common use cases
export const QuickUserForm: React.FC<{
  onSubmit: (data: z.infer<typeof userFormSchema>) => void
  defaultValues?: Partial<z.infer<typeof userFormSchema>>
  loading?: boolean
}> = ({ onSubmit, defaultValues, loading }) => {
  const fields: FormFieldConfig<z.infer<typeof userFormSchema>>[] = [
    { name: 'name', type: 'text', label: 'Full Name', placeholder: 'Enter your full name', required: true },
    { name: 'email', type: 'email', label: 'Email Address', placeholder: 'Enter your email', required: true },
    { name: 'password', type: 'password', label: 'Password', placeholder: 'Enter password', required: true },
    { name: 'confirmPassword', type: 'password', label: 'Confirm Password', placeholder: 'Confirm password', required: true },
    {
      name: 'role',
      type: 'select',
      label: 'Role',
      required: true,
      options: [
        { value: 'admin', label: 'Administrator' },
        { value: 'user', label: 'User' },
        { value: 'viewer', label: 'Viewer' },
      ],
    },
    { name: 'bio', type: 'textarea', label: 'Bio', placeholder: 'Tell us about yourself', rows: 3 },
    { name: 'agreeToTerms', type: 'checkbox', label: 'Terms and Conditions', placeholder: 'I agree to the terms and conditions', required: true },
  ]

  return (
    <FormBuilder
      fields={fields}
      schema={userFormSchema}
      onSubmit={onSubmit}
      defaultValues={defaultValues}
      loading={loading}
      gridCols={2}
    />
  )
}

export const QuickProjectForm: React.FC<{
  onSubmit: (data: z.infer<typeof projectFormSchema>) => void
  defaultValues?: Partial<z.infer<typeof projectFormSchema>>
  loading?: boolean
}> = ({ onSubmit, defaultValues, loading }) => {
  const fields: FormFieldConfig<z.infer<typeof projectFormSchema>>[] = [
    { name: 'name', type: 'text', label: 'Project Name', placeholder: 'Enter project name', required: true },
    { name: 'description', type: 'textarea', label: 'Description', placeholder: 'Describe your project', required: true, rows: 3 },
    {
      name: 'language',
      type: 'select',
      label: 'Programming Language',
      required: true,
      options: [
        { value: 'javascript', label: 'JavaScript' },
        { value: 'typescript', label: 'TypeScript' },
        { value: 'python', label: 'Python' },
        { value: 'java', label: 'Java' },
        { value: 'go', label: 'Go' },
        { value: 'rust', label: 'Rust' },
      ],
    },
    {
      name: 'framework',
      type: 'select',
      label: 'Framework',
      required: true,
      options: [
        { value: 'react', label: 'React' },
        { value: 'vue', label: 'Vue.js' },
        { value: 'angular', label: 'Angular' },
        { value: 'svelte', label: 'Svelte' },
        { value: 'nextjs', label: 'Next.js' },
        { value: 'express', label: 'Express.js' },
        { value: 'fastapi', label: 'FastAPI' },
        { value: 'django', label: 'Django' },
      ],
    },
    { name: 'repository', type: 'text', label: 'Repository URL', placeholder: 'https://github.com/...' },
    { name: 'private', type: 'checkbox', label: 'Private Project', placeholder: 'Make this project private' },
    { name: 'tags', type: 'text', label: 'Tags', placeholder: 'web, api, frontend (comma separated)' },
  ]

  return (
    <FormBuilder
      fields={fields}
      schema={projectFormSchema}
      onSubmit={onSubmit}
      defaultValues={defaultValues}
      loading={loading}
      gridCols={2}
    />
  )
}